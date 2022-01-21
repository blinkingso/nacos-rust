// use crate::cache::CacheData;
// use crate::client::SafeAccess;
// use crate::consts::names::*;
// use crate::consts::val::{
//     CONFIG_LONG_POLL_TIMEOUT as DEFAULT_CONFIG_LONG_POLL_TIMEOUT,
//     CONFIG_RETRY_TIME as DEFAULT_CONFIG_RETRY_TIME, DEFAULT_NAMESPACE,
//     MIN_CONFIG_LONG_POLL_TIMEOUT,
// };
// use crate::security::auth::login;
// use crate::security::SecurityProxy;
// use crate::{NacosResult, Properties};
// use chrono::Utc;
// use std::borrow::Borrow;
// use std::cmp::max;
// use std::collections::HashMap;
// use std::fmt::Debug;
// use std::hash::Hash;
// use std::option::Option;
// use std::str::FromStr;
// use std::sync::{Arc, Mutex, RwLock};
// use tokio::sync::mpsc::Receiver;
//
// const HTTPS: &'static str = "https://";
// const HTTP: &'static str = "http://";
//
// const BELL: u32 = 1;
//
// pub struct ClientWorker {
//     security_proxy: SafeAccess<SecurityProxy>,
//     enable_remote_sync_config: bool,
//     task_penalty_time: i32,
//     timeout: i32,
//     listen_execute_bell: Receiver<i32>,
// }
//
// pub struct NacosConfigClient<'a> {
//     properties: &'a Properties,
// }
//
// pub async fn start_client_worker(
//     security: SafeAccess<SecurityProxy>,
//     server_urls: &Vec<String>,
//     listen_execute_bell: &mut Receiver<u32>,
// ) -> NacosResult<()> {
//     {
//         let lock = &security.data.lock().unwrap();
//         if lock.enabled() {
//             let server_urls = server_urls.clone();
//             let security = security.clone();
//             tokio::spawn(async move {
//                 if crate::security::auth::login(&server_urls, security)
//                     .await
//                     .is_err()
//                 {
//                     warn!("login failed...");
//                 }
//             });
//         }
//     }
//
//     let server_urls = server_urls.clone();
//     let security = security.clone();
//     let (tx, mut rx) = tokio::sync::mpsc::channel(1);
//     tokio::spawn(async move {
//         loop {
//             tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
//             let tx = tx.clone();
//             let server_urls = server_urls.clone();
//             let _ = tx.send(server_urls).await;
//         }
//     });
//     tokio::spawn(async move {
//         while let Some(server_urls) = rx.recv().await {
//             let security = security.clone();
//             if crate::security::auth::login(&server_urls, security)
//                 .await
//                 .is_err()
//             {
//                 warn!("login failed...")
//             }
//         }
//     });
//
//     let mut config_rpc_client = ConfigRpcClient {
//         last_all_sync_time: Arc::new(Mutex::new(Utc::now().timestamp_millis())),
//         cache_map: Arc::new(RwLock::new(HashMap::new())),
//     };
//
//     config_rpc_client.start(listen_execute_bell).await
// }
// pub(crate) struct ConfigRpcClient {
//     pub(crate) last_all_sync_time: Arc<Mutex<i64>>,
//     pub(crate) cache_map: Arc<RwLock<HashMap<String, CacheData>>>,
// }
// const ALL_SYNC_INTERNAL: i64 = 5 * 60 * 1000;
//
// impl ConfigRpcClient {
//     async fn start(&mut self, listen_execute_bell: &mut Receiver<u32>) -> NacosResult<()> {
//         if listen_execute_bell.recv().await.is_some() {
//             let mut listen_cache: HashMap<String, Vec<CacheData>> = HashMap::new();
//             let mut remove_listen_cache: HashMap<String, Vec<CacheData>> = HashMap::new();
//             let now = Utc::now().timestamp_millis();
//             let need_all_sync = {
//                 let last_all_sync_time = self.last_all_sync_time.lock().unwrap();
//                 now - *last_all_sync_time > ALL_SYNC_INTERNAL
//             };
//
//             {
//                 let lock = self.cache_map.read().unwrap();
//                 for (_, cache_data) in &*lock {
//                     // if cache_data.is_sync_with_server {
//                     //     cache_data.check_listener_md5();
//                     //     if !need_all_sync {
//                     //         continue;
//                     //     }
//                     // }
//                 }
//             }
//
//             if need_all_sync {
//                 let mut lock = self.last_all_sync_time.lock().unwrap();
//                 *lock = now;
//             }
//         }
//         Ok(())
//     }
// }
// impl ClientWorker {
//     pub const BELL_ITEM: i32 = 0;
//     pub fn new(proxy: SecurityProxy, properties: &Properties, rx: Receiver<i32>) -> Self {
//         let mut client_worker = ClientWorker {
//             listen_execute_bell: rx,
//             security_proxy: SafeAccess::new(proxy),
//             enable_remote_sync_config: false,
//             task_penalty_time: 0,
//             timeout: 0,
//         };
//         // init settings here.
//         client_worker.init(properties);
//         client_worker
//     }
//     pub fn init(&mut self, properties: &Properties) {
//         let timeout = max(
//             get_property_or_default(
//                 properties,
//                 CONFIG_LONG_POLL_TIMEOUT,
//                 DEFAULT_CONFIG_LONG_POLL_TIMEOUT,
//             ),
//             MIN_CONFIG_LONG_POLL_TIMEOUT,
//         );
//         let task_penalty_time =
//             get_property_or_default(properties, CONFIG_RETRY_TIME, DEFAULT_CONFIG_RETRY_TIME);
//         let enable_remote_sync_config =
//             get_property_or_default(properties, ENABLE_REMOTE_SYNC_CONFIG, false);
//         self.enable_remote_sync_config = enable_remote_sync_config;
//         self.timeout = timeout;
//         self.task_penalty_time = task_penalty_time;
//     }
//
//     // pub async fn start(&mut self, properties: &Properties) -> NacosResult<()> {
//     //     {
//     //         let mut proxy = self.security_proxy.data.clone().lock().await;
//     //         if *proxy.enabled() {
//     //             let slm = ServerListManager::new(properties)?;
//     //             let mut sus = slm.server_urls.data.clone().lock().await;
//     //             let sus = sus.unwrap().clone();
//     //             if *proxy.login(&sus).await.is_err() {
//     //                 warn!("login failed.");
//     //             }
//     //
//     //             // loop login.
//     //             {
//     //                 // let server_urls = server_urls.clone();
//     //                 // let proxy = self.security_proxy.data.clone();
//     //                 // tokio::pin!(proxy);
//     //                 // tokio::pin!(server_urls);
//     //                 // tokio::spawn(async move {
//     //                 //     loop {
//     //                 //         let mut p = { (*proxy.lock().unwrap()).borrow_mut() };
//     //                 //         p.login(&server_urls).await;
//     //                 //     }
//     //                 // });
//     //             }
//     //         }
//     //     }
//     //     self.start_internal().await;
//     //     Ok(())
//     // }
//
//     async fn start_internal(&mut self) {
//         loop {
//             if let Some(_) = self.listen_execute_bell.recv().await {}
//         }
//     }
// }
//
// /// A struct to manager Server list.
// struct ServerListManager {
//     pub endpoint: Option<String>,
//     pub endpoint_port: Option<u16>,
//     pub context_path: Option<String>,
//     pub server_list_name: Option<String>,
//     pub namespace: Option<String>,
//     pub tenant: Option<String>,
//     pub address_server_url: SafeAccess<Option<String>>,
//     pub server_urls: SafeAccess<Option<Vec<String>>>,
//     pub is_started: SafeAccess<bool>,
//     pub server_addrs_str: SafeAccess<Option<String>>,
// }
//
// fn init_server_list_param(
//     properties: &Properties,
// ) -> (Option<String>, Option<u16>, Option<String>, Option<String>) {
//     let endpoint = match properties.get(ENDPOINT) {
//         Some(value) => Some(value.to_string()),
//         None => None,
//     };
//
//     let endpoint_port = properties
//         .get(ENDPOINT_PORT)
//         .unwrap_or(&DEFAULT_PORT.to_string())
//         .parse::<u16>()
//         .unwrap();
//
//     let content_path = match properties.get(CONTEXT_PATH) {
//         Some(value) => Some(value.to_string()),
//         None => None,
//     };
//     let server_list_name = match properties.get(CLUSTER_NAME) {
//         Some(value) => Some(value.to_string()),
//         None => None,
//     };
//     (
//         endpoint,
//         Some(endpoint_port),
//         content_path,
//         server_list_name,
//     )
// }
//
// impl ServerListManager {
//     pub fn new(properties: &Properties) -> NacosResult<Self> {
//         let is_started = SafeAccess::new(false);
//         let server_addrs_str = properties.get(SERVER_ADDR);
//         let namespace = if let Some(namespace) = properties.get(NAMESPACE) {
//             namespace.to_string()
//         } else {
//             DEFAULT_NAMESPACE.to_string()
//         };
//         return if server_addrs_str.is_some() {
//             let server_addrs: Vec<String> = server_addrs_str
//                 .unwrap()
//                 .split(",")
//                 .map(|address| {
//                     return if address.starts_with(HTTPS) | address.starts_with(HTTP) {
//                         address.to_string()
//                     } else {
//                         let ip_ports = address.split(":");
//                         if ip_ports.into_iter().count() == 1 {
//                             format!("{}{}:{}", HTTP, address.trim(), DEFAULT_PORT)
//                         } else {
//                             format!("{}{}", HTTP, address.trim())
//                         }
//                     };
//                 })
//                 .collect();
//             let tenant = if namespace.is_empty() {
//                 "".to_string()
//             } else {
//                 namespace.clone()
//             };
//             Ok(ServerListManager {
//                 endpoint: None,
//                 endpoint_port: None,
//                 context_path: None,
//                 server_list_name: None,
//                 namespace: Some(namespace),
//                 tenant: Some(tenant),
//                 address_server_url: SafeAccess::new(None),
//                 server_urls: SafeAccess::new(Some(server_addrs)),
//                 is_started,
//                 server_addrs_str: SafeAccess::new(Some(server_addrs_str.unwrap().to_string())),
//             })
//         } else {
//             // todo! when using endpoint to connect to nacos server.
//             let (endpoint, endpoint_port, content_path, server_list_name) =
//                 init_server_list_param(properties);
//             let endpoint_url = if endpoint.is_some() {
//                 format!("{}:{}", endpoint.unwrap(), endpoint_port.unwrap())
//             } else {
//                 "".to_string()
//             };
//
//             Ok(ServerListManager {
//                 endpoint: None,
//                 endpoint_port: None,
//                 context_path: None,
//                 server_list_name: None,
//                 namespace: None,
//                 tenant: None,
//                 address_server_url: SafeAccess::new(None),
//                 server_urls: SafeAccess::new(None),
//                 is_started: SafeAccess::new(false),
//                 server_addrs_str: SafeAccess::new(None),
//             })
//         };
//     }
// }
//
// fn get_property_or_default<T>(properties: &Properties, key: &str, default: T) -> T
// where
//     T: FromStr + Debug,
//     T::Err: Debug,
// {
//     if let Some(value) = properties.get(key) {
//         value.as_str().parse::<T>().unwrap()
//     } else {
//         default
//     }
// }
