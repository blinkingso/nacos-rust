//! GrpcConnection
use crate::grpc::util::{convert_request, convert_response, parse_request, parse_response};
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
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::DerefMut;
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::mpsc::{Receiver, Sender};
use tonic::transport::{Channel, Uri};
use tonic::{IntoRequest, Request, Response, Status, Streaming};
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_ip: String,
    pub server_port: u16,
    pub enable_ssl: bool,
}

pub struct GrpcConnection {
    pub connection_id: Option<String>,
    pub abandon: bool,
    pub server_info: ServerInfo,
    pub channel: Option<Channel>,
    // to observe channel stream.
    pub sender: Option<Sender<Request<Payload>>>,
    // client to handle Request/config
    pub request_stub: Option<RequestClient<Channel>>,
}

impl GrpcConnection {
    /// create a [GrpcConnection] instance.
    pub fn new(server_info: ServerInfo) -> Self {
        GrpcConnection {
            connection_id: None,
            abandon: false,
            server_info,
            channel: None,
            sender: None,
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
        let request = Request::new(payload);
        return if sender.send(request).await.is_ok() {
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
        let request = Request::new(payload);
        return if sender.send(request).await.is_ok() {
            Ok(())
        } else {
            log::error!("send response error.");
            Err(NacosError::msg("response send failed"))
        };
    }

    /// A function to send config to the queue to handle `rpc` [RpcRequest]
    pub async fn send_request_with_timeout<Req>(
        &self,
        request: Req,
        timeout_millis: u64,
    ) -> NacosResult<()>
    where
        Req: DerefMut<Target = RpcRequest> + Serialize,
    {
        let payload = convert_request::<Req>(&request);
        let mut req = payload.into_request();
        req.set_timeout(Duration::from_millis(timeout_millis));
        match self.sender.clone().unwrap().send(req).await {
            Ok(_) => {
                log::info!("send rpc config to worker queue successfully.")
            }
            Err(error) => {
                log::error!("send rpc config error to the worker queue, {}", error);
                return Err(NacosError::msg("send config failed."));
            }
        }
        Ok(())
    }
}
