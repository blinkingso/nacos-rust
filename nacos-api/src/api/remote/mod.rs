pub mod request;
pub mod response;

use serde::{Deserialize, Serialize};

/// Request Type Definition.
#[derive(Serialize, Deserialize)]
#[repr(C)]
pub enum RequestType {
    ConnectionSetupRequest = 0,
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub enum ResponseType {
    Response = 0,
}
