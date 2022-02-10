use crate::common::GroupKey;
use std::boxed::Box;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Subscription(GroupKey);
impl Subscription {
    fn new(group_key: GroupKey) -> Self {
        Subscription(group_key)
    }
}

type ListenerMap<T> = HashMap<Subscription, Box<dyn Fn(T) + Send + 'static>>;

/// Safety access to listeners, etc...
#[derive(Clone)]
pub struct ListenerSet<T>
where
    T: Send,
{
    listeners: Arc<Mutex<ListenerMap<T>>>,
}

impl<T> ListenerSet<T>
where
    T: Send + Clone,
{
    pub fn new() -> Self {
        ListenerSet {
            listeners: Arc::new(Mutex::new(ListenerMap::new())),
        }
    }

    /// create a new subscribe.
    pub fn subscribe<Listener: Fn(T) + Send + 'static>(
        &self,
        group_key: GroupKey,
        listener: Listener,
    ) -> Subscription {
        let mut lock = self.listeners.lock().unwrap();
        let subscription = Subscription::new(group_key);
        lock.insert(subscription.clone(), Box::new(listener));
        subscription
    }

    /// remove listener from listeners.
    pub fn unsubscribe(&self, subscription: Subscription) {
        let mut lock = self.listeners.lock().unwrap();
        lock.remove(&subscription);
    }

    pub fn notify(&self, payload: &T) {
        let listeners = self.listeners.lock().unwrap();
        for listener in listeners.values() {
            listener(payload.clone())
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }
}

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

/// Listener for watch config.
pub trait Listener {
    type Incoming;
    /// Receive config info.
    /// #Parameters
    /// * config_info [Self::Incoming] config info.
    /// #Returns
    /// Nothing.
    fn receive_config_info(&self, config_info: Self::Incoming);
}

pub trait ConnectionEventListener {
    /// notify when connected to server.
    fn on_connected(&self);
    /// notify when disconnected to server.
    fn on_disconnect(&self);
}

#[cfg(test)]
mod tests {
    use super::ListenerSet;
    use crate::common::GroupKey;
    use std::sync::mpsc;

    #[test]
    fn test_new_listener_set() {
        let ls = ListenerSet::<()>::new();
        assert_eq!(ls.len(), 0);
    }

    #[test]
    fn test_new_listener_for_chan() {
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("001", "pay").unwrap();
        ls.subscribe(gk, |data| {});
        assert_eq!(ls.len(), 1);
    }

    #[test]
    fn test_add_listener_to_set() {
        let (tx, rx) = mpsc::channel();
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("003", "pay").unwrap();
        ls.subscribe(gk, move |e| tx.send(e).unwrap());
        assert_eq!(ls.len(), 1);

        ls.notify(&true);
        assert!(rx.recv().is_ok());
    }

    #[test]
    fn test_remove_listener_from_set() {
        let (tx, rx) = mpsc::channel();
        let ls = ListenerSet::<bool>::new();
        let gk = GroupKey::new_without_tanant("003", "pay").unwrap();
        let sub = ls.subscribe(gk, move |e| tx.send(e).unwrap());
        ls.unsubscribe(sub);
        assert_eq!(ls.len(), 0);
        ls.notify(&true);
        assert!(rx.recv().is_err());
    }
}

pub mod config {
    use super::{ConfigChangeItem, PropertyChangeType};
    use nacos_core::error::{NacosError, NacosResult};
    use std::collections::HashMap;

    pub fn parse_change_data(
        last_content: &str,
        content: &str,
        ty: &str,
    ) -> HashMap<String, String> {
        todo!()
    }

    pub trait ConfigChangeParser {
        // judge type.
        fn is_responsible_for(&self, ty: String) -> bool;
        /// compare old and new data.
        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>>;

        fn filter_change_data(
            &self,
            old_map: &HashMap<String, String>,
            new_map: &HashMap<String, String>,
        ) -> HashMap<String, ConfigChangeItem> {
            let mut result: HashMap<String, ConfigChangeItem> = HashMap::with_capacity(16);
            for (key, val) in old_map {
                let mut cci: ConfigChangeItem;
                if new_map.contains_key(key) {
                    if val.eq(new_map.get(key).unwrap()) {
                        // no change key-value here.
                        continue;
                    }
                    cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: val.to_string(),
                        new_value: new_map.get(key).unwrap().clone(),
                        ty: PropertyChangeType::MODIFIED,
                    }
                } else {
                    // only old key exists.
                    cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: val.to_string(),
                        new_value: "".to_string(),
                        ty: PropertyChangeType::DELETED,
                    }
                }

                result.insert(key.to_string(), cci);
            }

            for (key, val) in new_map {
                if !old_map.contains_key(key) {
                    let cci = ConfigChangeItem {
                        key: key.to_string(),
                        old_value: "".to_string(),
                        new_value: val.to_string(),
                        ty: PropertyChangeType::ADDED,
                    };

                    result.insert(key.to_string(), cci);
                }
            }
            result
        }
    }

    pub struct PropertiesConfigChangeParser;
    pub struct YamlConfigChangeParser;
    impl PropertiesConfigChangeParser {
        fn parse_properties(&self, text: &str) -> NacosResult<HashMap<String, String>> {
            let mut result = HashMap::new();
            if !text.is_empty() {
                for line in text.lines().into_iter() {
                    if line.starts_with('#') {
                        continue;
                    }
                    let (k, v) = line
                        .split_once('=')
                        .ok_or(NacosError::msg("properties parse error"))?;
                    result.insert(k.to_string(), v.to_string());
                }
            }

            Ok(result)
        }
    }
    impl ConfigChangeParser for PropertiesConfigChangeParser {
        fn is_responsible_for(&self, ty: String) -> bool {
            "PROPERTIES".eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            let old_map = self.parse_properties(old_content)?;
            let new_map = self.parse_properties(new_content)?;

            Ok(self.filter_change_data(&old_map, &new_map))
        }
    }

    impl ConfigChangeParser for YamlConfigChangeParser {
        fn is_responsible_for(&self, ty: String) -> bool {
            "YAML".eq_ignore_ascii_case(ty.as_str())
        }

        fn do_parse(
            &self,
            old_content: &str,
            new_content: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            todo!()
        }
    }

    pub struct ConfigChangeHandler {
        pub parses: Vec<Box<dyn ConfigChangeParser>>,
    }
    impl ConfigChangeHandler {
        pub fn new() -> Self {
            let mut parses: Vec<Box<dyn ConfigChangeParser>> = Vec::new();
            parses.push(Box::new(PropertiesConfigChangeParser));
            parses.push(Box::new(YamlConfigChangeParser));
            ConfigChangeHandler { parses }
        }

        pub fn parse_change_data(
            &self,
            old_content: &str,
            new_content: &str,
            ty: &str,
        ) -> NacosResult<HashMap<String, ConfigChangeItem>> {
            for parser in self.parses.iter() {
                if parser.is_responsible_for(ty.to_string()) {
                    return parser.do_parse(old_content, new_content);
                }
            }

            Err(NacosError::msg("Unsupported config type parser."))
        }
    }
}
