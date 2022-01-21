pub mod names {
    pub const NAMESPACE: &'static str = "namespace";
    pub const SERVER_ADDR: &'static str = "server_addr";
    pub const ENDPOINT: &'static str = "endpoint";
    pub const ENDPOINT_PORT: &'static str = "endpoint_port";
    pub const CONTEXT_PATH: &'static str = "context_path";
    pub const CLUSTER_NAME: &'static str = "cluster_name";
    pub const SECRET_KEY: &'static str = "secret_key";
    pub const ACCESS_KEY: &'static str = "access_key";
    pub const RAM_ROLE_NAME: &'static str = "ram_role_name";
    pub const CONFIG_LONG_POLL_TIMEOUT: &'static str = "config_long_poll_timeout";
    pub const CONFIG_RETRY_TIME: &'static str = "config_retry_time";
    pub const MAX_RETRY: &'static str = "max_retry";
    pub const ENABLE_REMOTE_SYNC_CONFIG: &'static str = "enable_remote_syn_config";
    pub const USERNAME: &'static str = "username";
    pub const PASSWORD: &'static str = "password";
    pub const DEFAULT_PORT: &'static str = "8848";
    pub const BASE_PATH: &'static str = "/v1/cs";
    pub const CONFIG_CONTROLLER_PATH: &'static str = "/configs";
}
pub mod val {
    pub const CONFIG_LONG_POLL_TIMEOUT: i32 = 30000;
    pub const MIN_CONFIG_LONG_POLL_TIMEOUT: i32 = 10000;
    pub const CONFIG_RETRY_TIME: i32 = 1000;
    pub const DEFAULT_NAMESPACE: &'static str = "";
    pub const DEFAULT_GROUP: &'static str = "DEFAULT_GROUP";
}
pub mod res_names {
    pub const RESP_ACCESS_TOKEN: &'static str = "accessToken";
    pub const RESP_TOKEN_TTL: &'static str = "tokenTtl";
    pub const RESP_GLOBAL_ADMIN: &'static str = "globalAdmin";
}

pub mod path {
    pub const FILE_PATH_PREFIX: &'static str = "nacos/conf";
}
