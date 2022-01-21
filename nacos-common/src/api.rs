use std::collections::HashMap;
use std::env;
pub fn get_env(key: &str, default: &str) -> String {
    env::var(key).unwrap_or(default.to_string())
}
pub fn create_config_labels() -> HashMap<String, String> {
    let mut labels = HashMap::new();
    labels.insert(String::from("module"), "config".to_string());
    labels.insert(String::from("source"), "sdk".to_string());
    labels.insert(String::from("taskId"), get_env("TASK_ID", "0"));
    labels.insert(String::from("AppName"), get_env("APP_NAME", "unknown"));
    labels.insert(String::from("Vipserver-Tag"), get_env("VIP_SERVER_TAG", ""));
    labels.insert(String::from("Amory-Tag"), get_env("AMORY_TAG", ""));
    labels
}

pub mod ability {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientRemoteAbility {
        pub support_remote_connection: bool,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientConfigAbility {
        pub support_remote_metrics: bool,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ClientNamingAbility {
        pub support_delta_push: bool,
        pub support_remote_metric: bool,
    }

    #[derive(Debug, Clone, Serialize)]
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
}
