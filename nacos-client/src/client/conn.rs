//! GrpcConnection
use std::collections::HashMap;
use std::ops::DerefMut;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::{IntoRequest, IntoStreamingRequest, Request, Response, Status, Streaming};
use tonic::transport::{Channel, Uri};
use nacos_common::error::{NacosError, NacosResult};
use nacos_common::remote::request::{RpcRequest, ServerCheckRequest};
use nacos_common::remote::response::{RpcResponse, ServerCheckResponse};
use crate::grpc::grpc_proto::Payload;
use crate::grpc::grpc_proto::request_client::RequestClient;
use crate::grpc::util::{convert_request, parse_request, parse_response};
use log::{error, info};
use std::convert::TryFrom;
use std::str::FromStr;
use serde::de::DeserializeOwned;
use nacos_common::api::get_env;
use crate::grpc::grpc_proto::bi_request_stream_client::BiRequestStreamClient;

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
    pub sender: Sender<Payload>,
    // client to handle Request/request
    pub request_stub: RequestClient<Channel>,
}
pub fn rpc_port_offset() -> u16 {
    1000
}
impl GrpcConnection {

    pub async fn create_new_channel(server_ip: &str, server_port: u16) -> NacosResult<Channel> {
        let url = format!("{schema}://{ip}:{port}", schema="https", ip=server_ip, port=server_port);
        let uri = Uri::try_from(url.as_str())?;
        let mut endpoint = Channel::builder(uri);
        let channel = endpoint.keep_alive_timeout(Duration::from_millis(6 * 60 * 1000))
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .concurrency_limit(400)
            .tcp_nodelay(true)
            .connect().await?;
        Ok(channel)
    }

    async fn bind_request_stream(bi_request_stream_stub: BiRequestStreamClient<Channel>, grpc_conn: GrpcConnection) -> NacosResult<()> {
        let mut stream_stub = bi_request_stream_stub;
        // stream_stub.request_bi_stream().await?

        Ok(())
    }

    /// A function to instance [GrpConnection] and build a TCP connection.
    pub async fn connect_to_server(server_info: ServerInfo) -> NacosResult<GrpcConnection> {
        let port = server_info.server_port + rpc_port_offset();
        let channel = Self::create_new_channel(server_info.server_ip.as_str(), port).await?;
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<Payload>(512);
        tokio::spawn(async move {
            // a single thread to process received message from remote server.
                while let Some(payload) = receiver.recv().await {
                    // todo process messages from remote server.
                }
        });
        let request_stub = RequestClient::new(channel.clone());
        // server check.
        let server_check_response= match server_check(server_info.server_ip.as_str(), port, request_stub.clone()).await {
          Ok(response) => response,
            Err(error) => {
                drop(channel);
                error!("server check failed, {}", &error);
                return Err(NacosError::from(error));
            }
        };
        let bi_request_stream_stub = BiRequestStreamClient::new(channel.clone());

        let connection_id = server_check_response.connection_id.clone();
        let connection = GrpcConnection {
            connection_id: Some(connection_id),
            abandon: false,
            server_info,
            channel,
            sender,
            request_stub
        };
        Ok(connection)
    }
    pub async fn request<Req, Resp>(&mut self, request: Req, timeouts: usize) -> NacosResult<Resp> where Req: DerefMut<Target=RpcRequest> + Serialize, Resp: DerefMut<Target=RpcResponse> + DeserializeOwned{
        let payload = convert_request::<Req>(request);
        let req = payload.into_request();
        match self.request_stub.request(req).await {
            Ok(payload) => {
                info!("received response from remote : {:?}", payload);
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

/// RequestClient<Channel> is Clone.
pub async fn server_check(ip: &str, port: u16, request_client: RequestClient<Channel>) -> NacosResult<ServerCheckResponse> {
    let server_check_request = ServerCheckRequest {
        request: RpcRequest {
            headers: HashMap::new(),
            request_id: None
        }
    };
    let payload = convert_request::<ServerCheckRequest>(server_check_request);
    let mut request_client = request_client;
    let mut request = payload.into_request();
    request.set_timeout(Duration::from_millis(3000));
    let mut response = request_client.request(request).await?;
    let response = parse_response::<ServerCheckResponse>(&response.into_inner())?;
    Ok(response)
}
pub async fn bind_request_stream(stream_stub: BiRequestStreamClient<Channel>, connection: &GrpcConnection)  {
    // let mut stream_stub = stream_stub;
    // let mut response = stream_stub.request_bi_stream(()).await?;
    // let mut response = response.into_inner();
    // let payload = response.message().await?.unwrap();
    // let ty = payload.metadata.unwrap().r#type;
    unimplemented!()
}