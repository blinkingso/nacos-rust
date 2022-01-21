#[allow(dead_code)]
use crate::serde::{Deserialize, Serialize};
// path: get configs
const API_CONFIGS: &str = "/nacos/v1/cs/configs";

#[doc = "retrieve configs from server"]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNacosConfigs {
    pub tenant: String,
    pub data_id: String,
    pub group: String,
}
