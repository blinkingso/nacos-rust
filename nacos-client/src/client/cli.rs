//! A module to handle GrpcClient.
use crate::client::conn::{GrpcConnection, ServerInfo};
use crate::client::handlers::server::ServerRequestHandler;
use crate::grpc::util::{convert_request, parse_response};
use crate::listeners::ConnectionEventListener;
use nacos_common::api::ability::ClientAbilities;
use nacos_common::api::{create_config_labels, get_env};
use nacos_common::error::{NacosError, NacosResult};
use nacos_common::remote::request::{ConnectionSetupRequest, ServerCheckRequest};
use nacos_common::remote::response::ServerCheckResponse;
use nacos_proto::grpc::bi_request_stream_client::BiRequestStreamClient;
use nacos_proto::grpc::request_client::RequestClient;
use nacos_proto::grpc::Payload;
use nacos_proto::log_payload;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::time::Duration;
use tokio::sync::mpsc::{self, Sender};
use tonic::transport::{Channel, Uri};
use tonic::Request;

const KEEP_ALIVE_TIME: u64 = 5000;

pub struct GrpcClient {
    pub connection: Option<GrpcConnection>,
    pub tenant: Option<String>,
    pub client_abilities: ClientAbilities,
    pub labels: HashMap<String, String>,
    pub last_active_timestamp: u64,
    pub server_request_handlers: Vec<Box<dyn ServerRequestHandler>>,
    pub connection_event_listeners: Vec<Box<dyn ConnectionEventListener>>,
}

impl GrpcClient {
    pub async fn connect_to_server(&self, server_info: ServerInfo) -> NacosResult<GrpcConnection> {
        let channel = create_new_channel(&server_info).await?;
        let stub = RequestClient::new(channel.clone());
        // server check
        let connection_id = match server_check(stub).await {
            Ok(response) => response.connection_id,
            Err(error) => {
                log::error!("server check error, {}", error);
                return Err(NacosError::from(error));
            }
        };
        // bind bi config stream
        let mut grpc_conn = GrpcConnection::new(server_info);
        grpc_conn.connection_id = Some(connection_id);
        let bi_request_stream_stub = self.bind_request_stream(&channel).await?;
        grpc_conn.sender = Some(bi_request_stream_stub);
        grpc_conn.channel = Some((&channel).clone());
        grpc_conn.request_stub = Some(RequestClient::new(channel.clone()));
        let connection_setup_request = ConnectionSetupRequest {
            request: Default::default(),
            client_version: "Nacos-Rust-Sdk-0.1.0".to_string(),
            abilities: self.client_abilities.clone(),
            tenant: self.tenant.as_ref().unwrap_or(&"".to_string()).to_string(),
            labels: create_config_labels(),
        };
        grpc_conn.send_request(connection_setup_request).await?;
        Ok(grpc_conn)
    }

    async fn bind_request_stream(&self, channel: &Channel) -> NacosResult<Sender<Payload>> {
        let mut bi = BiRequestStreamClient::new(channel.clone());
        let (sender, mut receiver) = mpsc::channel::<Payload>(1024);
        // send a setup config.
        let outbound = async_stream::stream! {
            while let Some(payload) = receiver.recv().await {
                log_payload(&payload);
                yield payload;
            }
        };
        let mut request = Request::new(outbound);
        request.set_timeout(Duration::from_secs(15000));
        match bi.request_bi_stream(request).await {
            Ok(response_stream) => {
                let mut streaming = response_stream.into_inner();
                tokio::spawn(async move {
                    while let Ok(Some(payload)) = streaming.message().await {
                        log_payload(&payload);
                        // send response to server
                        let value = payload.body.as_ref().unwrap();
                        let metadata = payload.metadata.as_ref().unwrap();
                        let bytes = &value.value;
                        let ty = metadata.r#type.as_str();
                        log::warn!("type: {}", ty);
                        log::warn!("value: {}", String::from_utf8_lossy(bytes));
                        // todo send response to server.
                    }
                });
            }
            Err(error) => {
                error!("bi stream request error : {}", error);
            }
        }
        Ok(sender)
    }
}

async fn server_check(
    request_blocking_stub: RequestClient<Channel>,
) -> NacosResult<ServerCheckResponse> {
    let mut request_blocking_stub = request_blocking_stub;
    let server_check_request = ServerCheckRequest::new();
    let payload_request = convert_request::<ServerCheckRequest>(&server_check_request);
    let mut request = Request::new(payload_request);
    request.set_timeout(Duration::from_millis(3000));
    let mut response = request_blocking_stub.request(request).await?;
    let payload = response.into_inner();
    log_payload(&payload);
    let response = parse_response::<ServerCheckResponse>(&payload)?;
    Ok(response)
}

/// A function to calculate rpc port through offset to server port.
pub fn rpc_port_offset() -> u16 {
    1000
}
/// A function to create a new channel with specified [ServerInfo]
async fn create_new_channel(server_info: &ServerInfo) -> NacosResult<Channel> {
    const SCHEMA_HTTPS: &'static str = "https";
    const SCHEMA_HTTP: &'static str = "http";
    let ip = server_info.server_ip.as_str();
    let port = server_info.server_port + rpc_port_offset();
    let schema = if server_info.enable_ssl {
        SCHEMA_HTTPS
    } else {
        SCHEMA_HTTP
    };
    let url = format!(
        "{schema}://{ip}:{port}",
        schema = schema,
        ip = ip,
        port = port
    );
    let uri = Uri::try_from(url.as_str())?;
    let mut endpoint = Channel::builder(uri);
    let channel = endpoint
        .keep_alive_timeout(Duration::from_millis(keep_alive_time_mills()))
        .timeout(Duration::from_millis(timeout_mills()))
        .connect_timeout(Duration::from_millis(connect_timeout_mills()))
        .concurrency_limit(400)
        .tcp_nodelay(true)
        .connect()
        .await?;
    Ok(channel)
}

pub fn keep_alive_time_mills() -> u64 {
    const DEFAULT_KEEP_ALIVE_TIME_MILLS: u64 = 6 * 60 * 1000;
    let keep_alive_time = get_env(
        "nacos.remote.grpc.keep.alive.millis",
        DEFAULT_KEEP_ALIVE_TIME_MILLS.to_string().as_str(),
    );
    let keep_alive_time = keep_alive_time.parse::<u64>();
    match keep_alive_time {
        Ok(mills) => mills,
        Err(error) => {
            log::warn!(
                "property `nacos.remote.grpc.keep.alive.millis` is not a valid integer, {}",
                error
            );
            DEFAULT_KEEP_ALIVE_TIME_MILLS
        }
    }
}

pub fn timeout_mills() -> u64 {
    const DEFAULT_TIMEOUT_MILLS: u64 = 5000;
    let timeout = get_env(
        "nacos.remote.grpc.timeout.millis",
        DEFAULT_TIMEOUT_MILLS.to_string().as_str(),
    );
    let timeout = timeout.parse::<u64>();
    match timeout {
        Ok(mills) => mills,
        Err(error) => {
            log::warn!(
                "property `nacos.remote.grpc.timeout.millis` is not a valid integer, {}",
                error
            );
            DEFAULT_TIMEOUT_MILLS
        }
    }
}

pub fn connect_timeout_mills() -> u64 {
    const DEFAULT_CONNECT_TIMEOUT_MILLS: u64 = 5000;
    let connect_timeout = get_env(
        "nacos.remote.grpc.connect.timeout.millis",
        DEFAULT_CONNECT_TIMEOUT_MILLS.to_string().as_str(),
    );
    let connect_timeout = connect_timeout.parse::<u64>();
    match connect_timeout {
        Ok(mills) => mills,
        Err(error) => {
            log::warn!(
                "property `nacos.remote.grpc.connect.timeout.millis` is not a valid integer, {}",
                error
            );
            DEFAULT_CONNECT_TIMEOUT_MILLS
        }
    }
}

pub fn concurrency_limit() -> usize {
    const DEFAULT_CONCURRENCY_LIMIT: usize = 1024;
    let limit = get_env(
        "nacos.remote.grpc.concurrency.limit",
        DEFAULT_CONCURRENCY_LIMIT.to_string().as_str(),
    );
    let limit = limit.parse::<usize>();
    match limit {
        Ok(limit) => limit,
        Err(error) => {
            log::warn!(
                "property `nacos.remote.grpc.concurrency.limit` is not a valid integer, {}",
                error
            );
            DEFAULT_CONCURRENCY_LIMIT
        }
    }
}
