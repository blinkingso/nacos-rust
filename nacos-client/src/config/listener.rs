use nacos_api::api::config::listener::{ConfigChangeEvent, Listener};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign};
use std::sync::atomic::{AtomicUsize, Ordering};

pub trait AbstractConfigChangeListener: Listener {
    /// do nothing here
    fn receive_config_info(&self, _config_info: String) {}

    /// Handle config change.
    fn receive_config_change(&self, event: ConfigChangeEvent);
}

pub struct ConfigChangeListener {
    name: String,
    data_id: String,
    group: String,
}

static mut LISTENER_ID: AtomicUsize = AtomicUsize::new(0);

impl ConfigChangeListener {
    pub fn new(data_id: &str, group: &str) -> Self {
        let id = unsafe { LISTENER_ID.fetch_add(1, Ordering::SeqCst) };
        let name = format!("config-change-listener-{}", id);
        ConfigChangeListener {
            name,
            data_id: data_id.to_string(),
            group: group.to_string(),
        }
    }
}

impl PartialEq<Self> for ConfigChangeListener {
    fn eq(&self, other: &Self) -> bool {
        self.data_id == other.data_id && self.group == other.group && self.name == other.name
    }
}

impl Eq for ConfigChangeListener {}

impl Hash for ConfigChangeListener {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data_id.hash(state);
        self.group.hash(state);
        self.name.hash(state);
    }
}

#[test]
fn test_listeners() {
    let l1 = ConfigChangeListener::new("pay", "default");
    let l2 = ConfigChangeListener::new("pay2", "default");
    let l3 = ConfigChangeListener::new("pay", "default");
    let mut map = HashMap::new();
    map.insert(l1, "l1");
    map.insert(l2, "l3");
    map.insert(l3, "l3");
    assert_eq!(map.len(), 3);
}
