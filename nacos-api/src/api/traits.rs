use async_trait::async_trait;
use nacos_core::error::{NacosError, NacosResult};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

/// Rpc Request Extensions.
pub trait RequestExt: Serialize + DeserializeOwned {
    fn ty_name(&self) -> String;
    fn headers(&self) -> HashMap<String, String>;
    fn clear_headers(&mut self);
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}
/// Rpc Response Extensions.
pub trait ResponseExt: Serialize + DeserializeOwned {
    fn ty_name(&self) -> String;
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
    fn to_vec(&self) -> Vec<u8> {
        serde_json::to_vec(&self).unwrap()
    }
}
pub type Callback = Box<dyn RequestCallback + Send + Sync + 'static>;
pub trait RequestCallback {
    /// get timeout millis
    fn get_timeout(&self) -> u64;

    /// called on success
    fn on_response(&self, response: Vec<u8>);

    /// called on failed.
    fn on_exception(&self, error: NacosError);
}

#[async_trait]
pub trait Requester {
    /// send request to await.
    async fn request<Req, Resp>(&mut self, request: Req, timeout_mills: u64) -> NacosResult<Resp>
    where
        Req: RequestExt + Send + Sync + 'static,
        Resp: ResponseExt + Send + Sync + 'static;

    /// send request and process result background.
    async fn async_request<Req, Resp>(
        &mut self,
        request: Req,
        callback: Callback,
    ) -> NacosResult<()>
    where
        Req: RequestExt + Send + Sync + 'static,
        Resp: ResponseExt + Send + Sync + 'static;
}
