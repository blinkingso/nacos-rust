pub mod env;

use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientRemoteAbility {
    pub support_remote_connection: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigAbility {
    pub support_remote_metrics: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientNamingAbility {
    pub support_delta_push: bool,
    pub support_remote_metric: bool,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientAbilities {
    pub remote_ability: ClientRemoteAbility,
    pub config_ability: ClientConfigAbility,
    pub naming_ability: ClientNamingAbility,
}

impl Default for ClientAbilities {
    fn default() -> Self {
        ClientAbilities {
            remote_ability: ClientRemoteAbility {
                support_remote_connection: true,
            },
            config_ability: ClientConfigAbility {
                support_remote_metrics: true,
            },
            naming_ability: ClientNamingAbility {
                support_delta_push: false,
                support_remote_metric: false,
            },
        }
    }
}
