use crate::common::remote::client::conn::ServerInfo;
use chrono::Local;
use nacos_api::api::traits::{RequestExt, ResponseExt};
use serde::{Deserialize, Serialize};
use std::io::Bytes;
use std::ops::Deref;
use std::sync::atomic::{AtomicI64, AtomicIsize};
use std::sync::{Arc, Mutex};
pub const CONNECTED: isize = 1;
pub const DISCONNECTED: isize = 0;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionEvent {
    event_type: isize,
}
impl ConnectionEvent {
    pub fn new(event_type: isize) -> Self {
        ConnectionEvent { event_type }
    }
    pub fn is_connected(&self) -> bool {
        self.event_type == CONNECTED
    }

    pub fn is_disconnected(&self) -> bool {
        self.event_type == DISCONNECTED
    }
}
pub struct ReconnectContext {
    pub server_info: ServerInfo,
    pub on_request_fail: bool,
}
pub fn resolve_server_info(server_address: String) -> ServerInfo {
    const HTTP_PREFIX: &'static str = "http";
    const COLON: &'static str = ":";
    let server_port = std::env::var("nacos.server.port").unwrap_or("8848".to_string());
    let mut server_port = server_port.parse::<u16>().unwrap();
    if server_address.contains(HTTP_PREFIX) {
        let mut s = server_address.split(COLON);
        let _ = s.next().unwrap();
        let ip_address = s.next().unwrap().replace("//", "");
        if let Some(port) = s.next() {
            server_port = port.parse::<u16>().unwrap();
        }
        ServerInfo {
            server_ip: ip_address,
            server_port,
        }
    } else {
        let mut s = server_address.split(COLON);
        let ip = s.next().unwrap();
        if let Some(port) = s.next() {
            server_port = port.parse().unwrap();
        }

        ServerInfo {
            server_ip: ip.to_string(),
            server_port,
        }
    }
}
type ServerRequestHandler = Box<dyn Fn(String) -> Option<String> + Send + 'static>;
struct ServerRequestHandlerSet {
    handlers: Arc<Mutex<Vec<ServerRequestHandler>>>,
}
impl ServerRequestHandlerSet {
    pub fn new() -> ServerRequestHandlerSet {
        ServerRequestHandlerSet {
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn register(&mut self, handler: ServerRequestHandler) {
        let mut lock = self.handlers.lock().unwrap();
        lock.push(handler);
    }
}
pub struct RpcClient {
    name: String,
    pub last_active_timestamp: AtomicI64,
    server_request_handlers: ServerRequestHandlerSet,
}

impl RpcClient {
    pub async fn handle_server_request(&mut self, request: String) -> Option<String> {
        {
            *self.last_active_timestamp.get_mut() = Local::now().timestamp();
        }
        {
            let handlers = self.server_request_handlers.handlers.lock().unwrap();
            let request = request.to_string();
            for handle in handlers.iter() {
                let req = request.clone();
                let response = handle(req);
                if response.is_some() {
                    return Some(response.unwrap());
                }
            }
            return None;
        }
    }
}
