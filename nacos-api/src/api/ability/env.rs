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
