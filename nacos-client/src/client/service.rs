use crate::client::worker::ClientWorker;
use nacos_core::error::NacosResult;

pub trait ConfigService {
    /// Get nacos config
    /// # Params
    /// * data_id - dataId
    /// * group - group
    /// * timeout_ms - read timeout
    /// # Returns
    /// * config value
    fn get_config(&self, data_id: String, group: String, timeout_ms: i64) -> NacosResult<String>;

    /// Get config and register listener
    fn get_config_and_sign_listener(
        &mut self,
        data_id: String,
        group: String,
        timeout_ms: i64,
        listener: fn(String) -> (),
    );

    /// Add a listener to the configuration, after the server modified the configuration, the client will use the
    /// incoming listener callback.
    fn add_listener(
        &mut self,
        data_id: String,
        group: String,
        listener: fn(String) -> (),
    ) -> NacosResult<()>;

    /// Publish config.
    fn publish_config(&self, data_id: String, group: String, content: String) -> NacosResult<bool>;

    /// Publish config cas.
    fn publish_config_with_md5(
        &self,
        data_id: String,
        group: String,
        content: String,
        md5: String,
    ) -> NacosResult<bool>;

    /// Publish config cas with type.
    fn publish_config_with_md5_ty(
        &self,
        data_id: String,
        group: String,
        content: String,
        md5: String,
        r#type: String,
    ) -> NacosResult<bool>;

    /// Remove config.
    fn remove_config(&mut self, data_id: String, group: String) -> NacosResult<bool>;

    /// Remove listener.
    fn remove_listener(&mut self, data_id: String, group: String, listener: &fn(String) -> ());

    /// Get Server status.
    fn get_server_status(&self) -> String;

    /// Shutdown the resource service
    fn shutdown(self);
}

pub(crate) type ConfigFilterChainManager = fn(String) -> String;
pub struct NacosConfigService {
    pub(crate) worker: ClientWorker,
    pub(crate) namespace: String,
    /// non used here
    pub(crate) filter_chain: ConfigFilterChainManager,
}

impl NacosConfigService {
    const UP: &'static str = "UP";
    const DOWN: &'static str = "DOWN";
}
