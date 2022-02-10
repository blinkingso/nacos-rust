use crate::common::remote::client::conn::Connection;
use crate::common::remote::client::grpc::utils::{
    convert_request, convert_response, parse_response,
};
use nacos_api::api::traits::{Callback, RequestExt, Requester, ResponseExt};
use nacos_core::error::{NacosError, NacosResult};
use nacos_proto::grpc::request_client::RequestClient;
use nacos_proto::grpc::Payload;
use std::time::Duration;
use tokio::sync::mpsc;
use tonic::transport::Channel;
use tonic::Request;

pub struct GrpcConnection {
    pub conn: Connection,
    pub channel: Channel,
    payload_stream_observer: mpsc::Sender<Payload>,
    grpc_future_service_stub: RequestClient<Channel>,
}

#[async_trait::async_trait]
impl Requester for GrpcConnection {
    async fn request<Req, Resp>(
        &mut self,
        request: Req,
        timeout_mills: u64,
    ) -> nacos_core::error::NacosResult<Resp>
    where
        Req: RequestExt + Send + Sync + 'static,
        Resp: ResponseExt + Send + Sync + 'static,
    {
        let payload = convert_request(&request);
        let mut request = Request::new(payload);
        request.set_timeout(Duration::from_millis(timeout_mills));
        let response = self.grpc_future_service_stub.request(request).await?;
        let response = response.into_inner();
        let response: Resp = parse_response::<Resp>(&response)?;
        Ok(response)
    }

    async fn async_request<Req, Resp>(
        &mut self,
        request: Req,
        callback: Callback,
    ) -> nacos_core::error::NacosResult<()>
    where
        Req: RequestExt + Send + Sync + 'static,
        Resp: ResponseExt + Send + Sync + 'static,
    {
        let payload = convert_request(&request);
        let mut request = Request::new(payload);
        let mut client = self.grpc_future_service_stub.clone();
        let timeout = callback.get_timeout();
        request.set_timeout(Duration::from_millis(timeout));
        tokio::spawn(async move {
            let response = client.request(request).await;
            match response {
                Ok(resp) => callback.on_response(resp.into_inner().body.unwrap().value),
                Err(error) => {
                    callback.on_exception(NacosError::new(error));
                }
            }
        });
        Ok(())
    }
}

impl GrpcConnection {
    pub fn payload_stream_observer(&self) -> mpsc::Sender<Payload> {
        self.payload_stream_observer.clone()
    }

    pub fn grpc_future_service_stub(&self) -> RequestClient<Channel> {
        self.grpc_future_service_stub.clone()
    }

    pub async fn send_request<Req>(&self, request: Req) -> NacosResult<()>
    where
        Req: RequestExt,
    {
        let payload = convert_request(&request);
        let _ = self.payload_stream_observer().send(payload).await?;
        Ok(())
    }

    pub async fn send_response<Resp>(&self, response: Resp) -> NacosResult<()>
    where
        Resp: ResponseExt,
    {
        let payload = convert_response(&response);
        let _ = self.payload_stream_observer().send(payload).await?;
        Ok(())
    }
}
