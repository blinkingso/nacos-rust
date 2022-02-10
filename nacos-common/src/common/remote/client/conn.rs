#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub server_ip: String,
    pub server_port: u16,
}
pub struct Connection {
    pub connection_id: String,
    pub abandon: bool,
    pub server_info: ServerInfo,
}

/// rpc port offset, default 1000.
pub fn rpc_port_offset() -> u16 {
    1000
}

impl ServerInfo {
    pub fn new(ip: String, port: u16) -> Self {
        ServerInfo {
            server_ip: ip,
            server_port: port,
        }
    }

    pub fn rpc_offset_server_info(&self) -> Self {
        ServerInfo {
            server_ip: self.server_ip.clone(),
            server_port: self.server_port + rpc_port_offset(),
        }
    }
}
