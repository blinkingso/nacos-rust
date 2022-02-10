//! GrpcConnection
use crate::core::remote::{Connection, ConnectionMeta};
use crate::grpc::util::{convert_request, convert_response};
use nacos_api::api::remote::request::RpcRequest;
use nacos_api::api::remote::response::RpcResponse;
use nacos_core::error::{NacosError, NacosResult};
use nacos_proto::grpc::request_client::RequestClient;
use nacos_proto::grpc::Payload;
use nacos_proto::log_response;
use serde::Serialize;
use std::ops::DerefMut;
use std::time::Duration;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_ip: String,
    pub server_port: u16,
    pub enable_ssl: bool,
}

pub struct GrpcConnection {
    pub(crate) connection: Connection,
    pub(crate) connection_id: Option<String>,
    pub(crate) abandon: bool,
    pub(crate) server_info: ServerInfo,
    pub(crate) channel: Option<Channel>,
    // to observe channel stream.
    pub(crate) sender: Option<mpsc::Sender<Payload>>,
    // client to handle Request/config
    pub(crate) request_stub: Option<RequestClient<Channel>>,
}

impl GrpcConnection {
    /// create a [GrpcConnection] instance.
    pub fn new(server_info: ServerInfo) -> Self {
        GrpcConnection {
            connection: Default::default(),
            connection_id: None,
            abandon: false,
            server_info,
            channel: None,
            sender: None,
            request_stub: None,
        }
    }

    pub fn new_with_meta(
        server_info: ServerInfo,
        meta_info: ConnectionMeta,
        channel: Channel,
        sender: mpsc::Sender<Payload>,
    ) -> Self {
        let mut connection = Connection::default();
        connection.meta_info = Some(meta_info);
        GrpcConnection {
            connection,
            connection_id: None,
            abandon: false,
            server_info,
            channel: Some(channel),
            sender: Some(sender),
            request_stub: None,
        }
    }

    pub async fn request_timeout<Req>(
        &mut self,
        request: Req,
        timeout_millis: u64,
    ) -> NacosResult<()>
    where
        Req: DerefMut<Target = RpcRequest> + Serialize,
    {
        let payload = convert_request::<Req>(&request);
        let mut request = Request::new(payload);
        request.set_timeout(Duration::from_millis(timeout_millis));
        let resp = self.request_stub.as_mut().unwrap().request(request).await?;
        log_response(&resp);
        Ok(())
    }

    pub async fn send_request<Req>(&self, request: Req) -> NacosResult<()>
    where
        Req: DerefMut<Target = RpcRequest> + Serialize,
    {
        let sender = self.sender.as_ref().unwrap();
        let payload = convert_request::<Req>(&request);
        return if sender.send(payload).await.is_ok() {
            Ok(())
        } else {
            log::error!("send config error.");
            Err(NacosError::msg("config send failed"))
        };
    }

    pub async fn send_response<Resp>(&self, response: Resp) -> NacosResult<()>
    where
        Resp: DerefMut<Target = RpcResponse> + Serialize,
    {
        let sender = self.sender.as_ref().unwrap();
        let payload = convert_response::<Resp>(&response);
        return if sender.send(payload).await.is_ok() {
            Ok(())
        } else {
            log::error!("send response error.");
            Err(NacosError::msg("response send failed"))
        };
    }
}
