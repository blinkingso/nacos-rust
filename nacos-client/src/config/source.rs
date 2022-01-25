use crate::Properties;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NacosPropertySource {
    pub group_id: String,
    pub data_id: String,
    pub c_type: String,
    pub auto_refreshed: bool,
    pub properties: Properties,
}
