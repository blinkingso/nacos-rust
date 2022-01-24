use self::request::RpcRequest;
use async_trait::async_trait;
use std::ops::{DerefMut};
use serde::{Serialize};

macro_rules! impl_deref_mut {
    ($target: ty, $target_ty: ty, $target_ident: ident) => {
        impl std::ops::Deref for $target {
            type Target = $target_ty;
            fn deref(&self) -> &Self::Target {
                &self.$target_ident
            }
        }
        impl std::ops::DerefMut for $target {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$target_ident
            }
        }
    };
}

macro_rules! impl_deref_mut_request {
    (
        $($target: ty),+$(,)?
    ) => {
        $(
            impl_deref_mut!($target, RpcRequest, request);
        )+
    }
}

macro_rules! impl_deref_mut_response {
    (
        $($target: ty),+$(,)?
    ) => {
        $(
            impl_deref_mut!($target, RpcResponse, response);
        )+
    }
}
macro_rules! impl_internal_request {
   ($($target_ty:ty),+ $(,)?) => {
         $(
            impl  InternalRequest for $target_ty {}
            impl ClientRequest for $target_ty {}
        )*
    }
}

macro_rules! impl_server_request {
    ($($target_ty:ty),+ $(,)?) => {
         $(
            impl ServerRequest for $target_ty {}
        )*
    }
}

/// structs to handle communication from client to server.
pub mod request {
    use crate::api::ability::ClientAbilities;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    /// A mark trait to mark server request.
    pub trait ServerRequest {}
    /// A marker trait to mark client request.
    pub trait ClientRequest {}

    /// A internal request trait to mark  `internal` module.
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

    #[derive(Debug, Serialize, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RpcRequest {
        pub headers: HashMap<String, String>,
        pub request_id: Option<String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientDetectionRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectionSetupRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
        pub client_version: String,
        pub abilities: ClientAbilities,
        pub tenant: String,
        pub labels: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectResetRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
        pub server_ip: String,
        pub server_port: String,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HealthCheckRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PushAckRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
        pub request_id: String,
        pub success: bool,
        pub exception: Option<String>,
    }

    #[derive(Debug, Serialize, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConfigChangeNotifyRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
        pub data_id: String,
        pub group: String,
        pub tenant: Option<String>
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RequestMeta {
        pub connection_id: String,
        pub client_ip: String,
        pub client_version: String,
        pub labels: HashMap<String, String>,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerCheckRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
    }

    impl ServerCheckRequest {
        pub fn new() -> Self {
            let request = RpcRequest::default();
            ServerCheckRequest {
                request
            }
        }
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerLoaderInfoRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
    }

    #[derive(Debug, Serialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerReloadRequest {
        #[serde(flatten)]
        pub request: RpcRequest,
        pub reload_count: usize,
        pub reload_server: String,
    }

    impl Default for RpcRequest {
        fn default() -> Self {
            RpcRequest {
                headers: Default::default(),
                request_id: None,
            }
        }
    }

    impl_deref_mut_request!(
        ServerCheckRequest,
        HealthCheckRequest,
        ConnectionSetupRequest,
        ConfigChangeNotifyRequest,
    );

    impl_internal_request!(
        ServerReloadRequest,
        ServerLoaderInfoRequest,
        ServerCheckRequest,
        PushAckRequest,
        HealthCheckRequest,
        ConnectionSetupRequest,
    );
    impl_server_request!(ConnectResetRequest, ClientDetectionRequest, ConfigChangeNotifyRequest);
    impl ConfigRequest for ConfigChangeNotifyRequest{}
}

/// structs to handle communication from server to client.
pub mod response {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::ops::{Deref, DerefMut};

    const CODE_SUCCESS: u16 = 200;
    const CODE_FAIL: u16 = 500;

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct ResponseCode {
        pub code: u16,
        pub desc: &'static str,
    }

    impl ResponseCode {
        pub const SUCCESS: ResponseCode = ResponseCode {
            code: 200,
            desc: "Response ok",
        };
        pub const FAIL: ResponseCode = ResponseCode {
            code: 500,
            desc: "Response fail",
        };

        pub fn from_u16(code: u16) -> Option<ResponseCode> {
            match code {
                CODE_SUCCESS => Some(ResponseCode::SUCCESS),
                CODE_FAIL => Some(ResponseCode::FAIL),
                _ => None,
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RpcResponse {
        pub result_code: u16,
        pub error_code: u32,
        pub message: Option<String>,
        pub request_id: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientDetectionResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ConnectResetResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
    }

    impl ErrorResponse {
        pub fn build(error_code: u32, msg: String) -> ErrorResponse {
            let mut response = RpcResponse {
                result_code: CODE_SUCCESS,
                error_code,
                message: Some(msg),
                request_id: None,
            };
            ErrorResponse { response }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HealthCheckResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerCheckResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
        pub connection_id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerLoaderInfoResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
        pub address: String,
        pub loader_metrics: HashMap<String, Option<String>>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct ServerReloadResponse {
        #[serde(flatten)]
        pub response: RpcResponse,
    }

    impl_deref_mut_response!(
        ClientDetectionResponse,
        ConnectResetResponse,
        ErrorResponse,
        HealthCheckResponse,
        ServerCheckResponse,
        ServerLoaderInfoResponse,
        ServerReloadResponse
    );
}

#[async_trait]
pub trait Requester {
    type Item: Serialize;
    type Req: DerefMut<Target = RpcRequest>;
}