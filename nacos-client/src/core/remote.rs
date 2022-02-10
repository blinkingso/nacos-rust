use chrono::{DateTime, Local};
use nacos_api::api::ability::ClientAbilities;
use nacos_api::api::consts::remote::{LABEL_SOURCE, LABEL_SOURCE_CLUSTER, LABEL_SOURCE_SDK};
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub struct ConnectionMeta {
    pub connect_type: String,
    pub client_ip: String,
    pub remote_ip: String,
    pub remote_port: u16,
    pub local_port: u16,
    pub version: String,
    pub connection_id: Option<String>,
    pub create_time: DateTime<Local>,
    pub last_active_time: u64,
    pub app_name: String,
    pub tenant: Option<String>,
    pub labels: HashMap<String, String>,
}

impl ConnectionMeta {
    /// Check if this connection is sdk source.
    pub fn is_sdk_source(&self) -> bool {
        let source = self.labels.get(LABEL_SOURCE);
        source.is_some() && source.unwrap().eq_ignore_ascii_case(LABEL_SOURCE_SDK)
    }

    pub fn is_cluster_source(&self) -> bool {
        let source = self.labels.get(LABEL_SOURCE);
        source.is_some() && source.unwrap().eq_ignore_ascii_case(LABEL_SOURCE_CLUSTER)
    }
}

impl ToString for ConnectionMeta {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub struct Connection {
    pub traced: bool,
    pub abilities: ClientAbilities,
    pub meta_info: Option<ConnectionMeta>,
}

impl Connection {
    /// Update last Active Time to now.
    pub fn fresh_active_time(&mut self) {
        if let Some(ref mut meta) = self.meta_info {
            meta.last_active_time = Local::now().timestamp() as u64;
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        Connection {
            traced: false,
            abilities: ClientAbilities::default(),
            meta_info: None,
        }
    }
}
