#![allow(dead_code)]

/// structs to handle communication from client to server.
use crate::api::ability::ClientAbilities;
use crate::{impl_config_request, impl_internal_request, impl_req_ext, impl_server_request};
use serde::{Deserialize, Serialize};

use crate::api::traits::RequestExt;
use std::collections::HashMap;

/// A mark trait to mark server config.
pub trait ServerRequest {}
/// A internal config trait to mark  `internal` module.
pub trait InternalRequest {
    fn get_module(&self) -> String {
        String::from("internal")
    }
}

pub trait ConfigRequest {
    fn get_module(&self) -> String {
        String::from("config")
    }
}

pub trait NamingRequest {
    fn get_module(&self) -> String {
        String::from("naming")
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcRequest {
    pub headers: HashMap<String, String>,
    pub request_id: Option<String>,
}
impl Default for RpcRequest {
    fn default() -> Self {
        RpcRequest {
            headers: Default::default(),
            request_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientDetectionRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionSetupRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub client_version: String,
    pub abilities: ClientAbilities,
    pub tenant: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResetRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub server_ip: String,
    pub server_port: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PushAckRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub request_id: String,
    pub success: bool,
    pub exception: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigChangeNotifyRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub data_id: String,
    pub group: String,
    pub tenant: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RequestMeta {
    pub connection_id: String,
    pub client_ip: String,
    pub client_version: String,
    pub labels: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerCheckRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
}

impl ServerCheckRequest {
    pub fn new() -> Self {
        let request = RpcRequest::default();
        ServerCheckRequest { inner: request }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLoaderInfoRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerReloadRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub reload_count: usize,
    pub reload_server: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigListenContext {
    pub group: String,
    pub md5: Option<String>,
    pub data_id: String,
    pub tenant: Option<String>,
}

impl ConfigListenContext {
    pub fn new(
        group: String,
        md5: Option<String>,
        data_id: String,
        tenant: Option<String>,
    ) -> Self {
        ConfigListenContext {
            group,
            md5,
            data_id,
            tenant,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigBatchListenRequest {
    #[serde(flatten)]
    pub inner: RpcRequest,
    pub listen: bool,
    pub config_listen_contexts: Vec<ConfigListenContext>,
}

impl ConfigBatchListenRequest {
    pub fn new(
        inner: RpcRequest,
        listen: bool,
        config_listen_contexts: Vec<ConfigListenContext>,
    ) -> Self {
        ConfigBatchListenRequest {
            inner,
            listen,
            config_listen_contexts,
        }
    }
}

impl Default for ConfigBatchListenRequest {
    fn default() -> Self {
        ConfigBatchListenRequest {
            inner: Default::default(),
            listen: true,
            config_listen_contexts: vec![],
        }
    }
}

impl RequestExt for RpcRequest {
    fn ty_name(&self) -> String {
        "RpcRequest".to_string()
    }

    fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }

    fn clear_headers(&mut self) {
        self.headers.clear()
    }
}

impl_internal_request!(
    ServerReloadRequest,
    ServerLoaderInfoRequest,
    ServerCheckRequest,
    PushAckRequest,
    HealthCheckRequest,
    ConnectionSetupRequest,
);

impl_server_request!(
    ConnectResetRequest,
    ClientDetectionRequest,
    ConfigChangeNotifyRequest
);
impl_config_request! {
    ConfigBatchListenRequest,
    ConfigChangeNotifyRequest,
}

impl_req_ext! {
    ServerCheckRequest,
    HealthCheckRequest,
    ConnectionSetupRequest,
    ConfigChangeNotifyRequest,
    ConfigBatchListenRequest,
}
