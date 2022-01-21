use super::ty::ConfigType;
use serde::Deserialize;

/// A struct to store Nacos configuration read from .toml Configuration files.
#[derive(Debug, Deserialize)]
pub struct NacosConfigProperties {
    pub server_addrs: Option<Vec<String>>,
    pub context_path: Option<String>,
    pub encode: Option<String>,
    pub endpoint: Option<String>,
    pub namespace: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub auto_refreshed: Option<bool>,
    pub data_ids: Option<Vec<String>>,
    pub group: Option<String>,
    #[serde(borrow)]
    pub ty: Option<ConfigType<'static>>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub max_retry: Option<String>,
    pub long_poll_timeout: Option<String>,
    pub retry_time: Option<String>,
    pub enable_remote_sync_config: Option<bool>,
}

impl Default for NacosConfigProperties {
    fn default() -> Self {
        NacosConfigProperties {
            server_addrs: Some(vec!["127.0.0.1:8848".to_string()]),
            context_path: Some("".to_string()),
            encode: Some("UTF-8".to_string()),
            endpoint: None,
            namespace: None,
            access_key: None,
            secret_key: None,
            auto_refreshed: Some(false),
            data_ids: None,
            group: Some("DEFAULT_GROUP".to_string()),
            ty: Some(ConfigType::PROPERTIES),
            username: None,
            password: None,
            max_retry: None,
            long_poll_timeout: None,
            retry_time: None,
            enable_remote_sync_config: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_deserialize() {}
}
