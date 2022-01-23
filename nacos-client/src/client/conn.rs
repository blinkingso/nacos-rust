//! GrpcConnection
use crate::grpc::util::{convert_request, parse_request, parse_response};
use log::{error, info};
use nacos_common::api::{create_config_labels, get_env};
use nacos_common::error::{NacosError, NacosResult};
use nacos_common::remote::request::{
    ConnectionSetupRequest, HealthCheckRequest, RpcRequest, ServerCheckRequest,
};
use nacos_common::remote::response::{
    ConnectResetResponse, HealthCheckResponse, RpcResponse, ServerCheckResponse,
};
use nacos_proto::grpc::bi_request_stream_client::BiRequestStreamClient;
use nacos_proto::grpc::request_client::RequestClient;
use nacos_proto::grpc::{Metadata, Payload};
use nacos_proto::{log_payload, log_response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::transport::{Channel, Uri};
use tonic::{IntoRequest, IntoStreamingRequest, Request, Response, Status, Streaming};

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_ip: String,
    pub server_port: u16,
}
pub struct GrpcConnection {
    pub connection_id: Option<String>,
    pub abandon: bool,
    pub server_info: ServerInfo,
    pub channel: Channel,
    // to observe channel stream.
    pub sender: Option<Sender<Payload>>,
    // client to handle Request/request
    pub request_stub: RequestClient<Channel>,
}
pub fn rpc_port_offset() -> u16 {
    1000
}
impl GrpcConnection {
    pub async fn create_new_channel(server_ip: &str, server_port: u16) -> NacosResult<Channel> {
        let url = format!(
            "{schema}://{ip}:{port}",
            schema = "https",
            ip = server_ip,
            port = server_port
        );
        let uri = Uri::try_from(url.as_str())?;
        let mut endpoint = Channel::builder(uri);
        let channel = endpoint
            .keep_alive_timeout(Duration::from_millis(6 * 60 * 1000))
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .concurrency_limit(400)
            .tcp_nodelay(true)
            .connect()
            .await?;
        Ok(channel)
    }

    /// A function to instance [GrpConnection] and build a TCP connection.
    pub async fn connect_to_server(server_info: ServerInfo) -> NacosResult<GrpcConnection> {
        let port = server_info.server_port + rpc_port_offset();
        let channel = Self::create_new_channel(server_info.server_ip.as_str(), port).await?;
        let mut request_stub = RequestClient::new(channel.clone());
        let bi_channel = channel.clone();
        let request_stub_clone = request_stub.clone();
        let mut connection = GrpcConnection {
            connection_id: None,
            abandon: false,
            server_info: server_info.clone(),
            channel: channel.clone(),
            sender: None,
            request_stub: request_stub_clone,
        };
        // server check.
        let server_check_response =
            match server_check(server_info.server_ip.as_str(), port, &connection).await {
                Ok(response) => response,
                Err(error) => {
                    drop(channel);
                    error!("server check failed, {}", &error);
                    return Err(NacosError::from(error));
                }
            };

        let connection_id = server_check_response.connection_id.clone();
        connection.connection_id = Some(connection_id);

        let sender = bind_request_stream(bi_channel).await?;
        connection.sender = Some(sender);
        // send ConnectSetupRequest;
        let connection_setup_request = ConnectionSetupRequest {
            request: Default::default(),
            client_version: "Nacos-Rust-sdk.0.0.1".to_string(),
            abilities: Default::default(),
            tenant: "".to_string(),
            labels: create_config_labels(),
        };
        let _ = send_request::<ConnectionSetupRequest, ConnectResetResponse>(
            &connection,
            connection_setup_request,
            10,
        )
        .await?;
        Ok(connection)
    }

    pub async fn send_request_with_timeout<Req, Resp>(
        &self,
        request: Req,
        timeouts: u64,
    ) -> NacosResult<Resp>
    where
        Req: DerefMut<Target = RpcRequest> + Serialize,
        Resp: DerefMut<Target = RpcResponse> + DeserializeOwned,
    {
        let payload = convert_request::<Req>(request);
        let mut req = payload.into_request();
        req.set_timeout(Duration::from_secs(timeouts));
        let mut stub = conn.request_stub.clone();
        match stub.request(req).await {
            Ok(payload) => {
                log_response(&payload);
                let payload = payload.into_inner();
                let resp: Resp = parse_response::<Resp>(&payload)?;
                Ok(resp)
            }
            Err(ref error_status) => {
                error!("failed to request to remote server, {}", error_status);
                return Err(NacosError::msg(error_status.message().to_string()));
            }
        }
    }
}

pub async fn bind_request_stream(channel: Channel) -> NacosResult<Sender<Payload>> {
    let mut bi = BiRequestStreamClient::new(channel);
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<Payload>(1024);
    tokio::spawn(async move {
        while let Some(payload) = receiver.recv().await {
            tokio::spawn(async move {
                let response = bi
                    .request_bi_stream(futures_util::stream::iter([payload]))
                    .await?;
                let mut stream = response.into_inner();

                while let Some(payload) = stream.message().await? {
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
            })
        }
    });
    Ok(sender)
}

/// RequestClient<Channel> is Clone.
pub async fn server_check(
    ip: &str,
    port: u16,
    connection: &GrpcConnection,
) -> NacosResult<ServerCheckResponse> {
    let server_check_request = ServerCheckRequest {
        request: RpcRequest {
            headers: HashMap::new(),
            request_id: None,
        },
    };
    let mut response = send_request::<ServerCheckRequest, ServerCheckResponse>(
        connection,
        server_check_request,
        3,
    )
    .await?;
    Ok(response)
}

pub async fn health_check(
    ip: &str,
    port: u16,
    connection: &GrpcConnection,
) -> NacosResult<HealthCheckResponse> {
    let health_check_request = HealthCheckRequest {
        request: RpcRequest {
            headers: HashMap::new(),
            request_id: None,
        },
    };
    let mut response = send_request::<HealthCheckRequest, HealthCheckResponse>(
        connection,
        health_check_request,
        3,
    )
    .await?;
    Ok(response)
}
