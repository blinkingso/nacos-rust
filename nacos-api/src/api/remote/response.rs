use crate::impl_resp_ext;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const CODE_SUCCESS: u16 = 200;
const CODE_FAIL: u16 = 500;

#[derive(Serialize, Deserialize, Clone)]
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcResponse {
    pub result_code: u16,
    pub error_code: u32,
    pub message: Option<String>,
    pub request_id: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ClientDetectionResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResetResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ErrorResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

impl ErrorResponse {
    pub fn build(error_code: u32, msg: String) -> ErrorResponse {
        let response = RpcResponse {
            result_code: CODE_SUCCESS,
            error_code,
            message: Some(msg),
            request_id: None,
        };
        ErrorResponse { inner: response }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthCheckResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerCheckResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
    pub connection_id: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerLoaderInfoResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
    pub address: String,
    pub loader_metrics: HashMap<String, Option<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ServerReloadResponse {
    #[serde(flatten)]
    pub inner: RpcResponse,
}

impl_resp_ext! {
    ClientDetectionResponse,
    ConnectResetResponse,
    ErrorResponse,
    HealthCheckResponse,
    ServerCheckResponse,
    ServerLoaderInfoResponse,
    ServerReloadResponse,
    Response,
}
