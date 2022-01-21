extern crate config as toml;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate lazy_static;
extern crate state;
extern crate tonic;
extern crate prost_types;
extern crate prost;

mod cache;
pub mod client;
mod common;
mod config;
mod consts;
mod crypto;
mod http;
mod listeners;
mod security;
mod utils;
mod grpc;

use std::collections::HashMap;
pub type Properties = HashMap<String, String>;