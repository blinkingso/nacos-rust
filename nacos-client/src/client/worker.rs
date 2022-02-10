use crate::client::service::ConfigFilterChainManager;
use crate::common::GroupKey;
use crate::config::cache::CacheData;
use lazy_static::lazy_static;
use nacos_api::api::consts::{names, val};
use std::cmp::max;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref CACHE_MAP: RwLock<HashMap<String, Arc<CacheData>>> = RwLock::new(HashMap::new());
}

pub(crate) struct ClientWorker {
    // ConfigFilterChainManager todo.
    pub(crate) is_health_server: AtomicBool,
    pub(crate) config_filter_chain_manager: Option<ConfigFilterChainManager>,
    pub(crate) timeout: i32,
    pub(crate) task_penalty_time: i32,
    pub(crate) enable_remote_sync_config: bool,
}

impl Default for ClientWorker {
    fn default() -> Self {
        ClientWorker {
            is_health_server: Default::default(),
            config_filter_chain_manager: None,
            timeout: 0,
            task_penalty_time: 0,
            enable_remote_sync_config: false,
        }
    }
}

impl ClientWorker {
    const NOTIFY_HEADER: &'static str = "notify";
    const TAG_PARAM: &'static str = "tag";
    const APP_NAME_PARAM: &'static str = "appName";
    const BETA_IPS_PARAM: &'static str = "betaIps";
    const TYPE_PARAM: &'static str = "type";
    const ENCRYPTED_DATA_KEY_PARAM: &'static str = "encryptedDataKey";
    const DEFAULT_RESOURCE: &'static str = "";

    fn init(&mut self, properties: &HashMap<String, String>) {
        let timeout = if let Some(timeout) = properties.get(names::CONFIG_LONG_POLL_TIMEOUT) {
            timeout.parse().unwrap()
        } else {
            val::CONFIG_LONG_POLL_TIMEOUT
        };
        let timeout = max(timeout, val::MIN_CONFIG_LONG_POLL_TIMEOUT);
        self.timeout = timeout;
        let task_penalty_time = if let Some(time) = properties.get(names::CONFIG_RETRY_TIME) {
            time.parse().unwrap()
        } else {
            val::CONFIG_RETRY_TIME
        };
        self.task_penalty_time = task_penalty_time;
        let enable_remote_sync_config =
            if let Some(config) = properties.get(names::ENABLE_REMOTE_SYNC_CONFIG) {
                config.parse().unwrap()
            } else {
                false
            };
        self.enable_remote_sync_config = enable_remote_sync_config;
    }

    pub fn new(
        filter_chain: ConfigFilterChainManager,
        properties: HashMap<String, String>,
    ) -> ClientWorker {
        let mut client_worker = ClientWorker::default();
        // init properties.
        client_worker.config_filter_chain_manager = Some(filter_chain);
        client_worker.init(&properties);
        // create ConfigRpcTransportClient.

        client_worker
    }

    fn add_listeners(&mut self, data_id: String, group: String, listeners: Vec<fn(String) -> ()>) {
        let group = blank2_default_group(group);
        let cache = add_cache_data_if_absent(data_id, group);
    }
}

fn blank2_default_group(group: String) -> String {
    if group.is_empty() || group.trim().is_empty() {
        val::DEFAULT_GROUP.to_string()
    } else {
        group.trim().to_string()
    }
}
fn get_cache(data_id: &str, group: &str) -> Option<Arc<CacheData>> {
    let tenant = tenant::get_user_tenant_for_acm();
    let group_key = GroupKey::new(data_id, group, tenant.as_str()).unwrap();
    let group_key = group_key.to_string();
    let cache = {
        let lock = CACHE_MAP.read().unwrap();
        let cache = lock.get(group_key.as_str());
        if let Some(cache) = cache {
            Some(cache.clone())
        } else {
            None
        }
    };
    cache
}
fn add_cache_data_if_absent(data_id: String, group: String) -> Option<Arc<CacheData>> {
    let cache = get_cache(data_id.as_str(), group.as_str());
    return if let Some(cache) = cache {
        Some(cache)
    } else {
        let key = GroupKey::new_without_tenant(data_id.as_str(), group.as_str()).unwrap();
        let cache = CacheData {};
        let task_id = { CACHE_MAP.read().unwrap().len() / 3000 };
        {
            let mut write = CACHE_MAP.write().unwrap();
            let cache = get_cache(data_id.as_str(), group.as_str());
        }
        None
    };
}

mod tenant {
    use lazy_static::lazy_static;
    const ACM_NAMESPACE_PROPERTY: &'static str = "acm.namespace";
    const DEFAULT_ACM_NAMESPACE: &'static str = "";
    lazy_static! {
        static ref USER_TENANT: String = { std::env::var("tenant.id").unwrap_or("".to_string()) };
    }

    pub fn get_user_tenant_for_acm() -> String {
        let mut tmp = USER_TENANT.clone();

        if tmp.trim().is_empty() {
            tmp =
                std::env::var(ACM_NAMESPACE_PROPERTY).unwrap_or(DEFAULT_ACM_NAMESPACE.to_string());
        }

        tmp
    }
}

#[test]
fn test_read_cache() {
    {
        CACHE_MAP
            .write()
            .unwrap()
            .insert("hello".to_string(), Arc::new(CacheData));
    }
    let read = CACHE_MAP.read().unwrap();
    let data = read.get("hello");
    if let Some(_) = data {
        assert!(true);
    } else {
        assert!(false);
    }
}
