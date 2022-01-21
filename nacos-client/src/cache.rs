use lazy_static::lazy_static;
use serde::Deserialize;
use state::Storage;
use std::boxed::Box;
use std::sync::RwLock;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigResponse {
    pub tenant: String,
    pub data_id: String,
    pub group: String,
    pub content: String,
    pub config_type: String,
    pub encrypted_data_key: String,
}

lazy_static! {
    static ref CONFIG_LISTENERS: Storage<RwLock<Box<dyn Fn(String) + Send + Sync + 'static>>> =
        Storage::new();
}

pub struct CacheData {
    name: String,
    data_id: String,
    group: String,
    tenant: String,
    md5: String,
    is_use_local_config: bool,
    local_config_last_modified: u64,
    content: String,
    encrypted_data_key: String,
    last_modified_ts: u64,
    task_id: String,
    ty: String,
    is_init: bool,
    is_sync_with_server: bool,
}

impl CacheData {
    pub fn check_listener_md5(&self) {}
}
