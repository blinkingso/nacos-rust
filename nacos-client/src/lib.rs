extern crate config as toml;
#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate pretty_env_logger;
extern crate prost;
extern crate prost_types;
extern crate state;
extern crate tonic;

mod cache;
pub mod client;
mod common;
mod config;
pub mod core;
mod crypto;
mod grpc;
mod http;
mod listeners;
mod security;
mod utils;

use std::collections::HashMap;
pub type Properties = HashMap<String, String>;
