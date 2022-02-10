use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PropertyChangeType {
    ADDED = 0,
    MODIFIED = 1,
    DELETED = 2,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConfigChangeItem {
    pub key: String,
    pub old_value: String,
    pub new_value: String,
    pub ty: PropertyChangeType,
}

pub struct ConfigChangeEvent {
    pub data: HashMap<String, ConfigChangeItem>,
}

pub trait Listener: Eq + Hash {
    /// Receive config info.
    fn receive_config_info(&self, config_info: String);
}
