//! A module to define structs for common request and response

use nacos_api::api::remote::{RequestType, ResponseType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Payload Structure.
#[derive(Serialize, Deserialize)]
pub struct GRpcRequest {
    pub r#type: RequestType,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}

/// Response payload Structure
#[derive(Serialize, Deserialize)]
pub struct GRpcResponse {
    pub r#type: ResponseType,
    pub body: Vec<u8>,
    pub headers: HashMap<String, String>,
}
