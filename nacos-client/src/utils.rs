use crate::config::props::NacosConfigProperties;
use nacos_common::error::NacosResult;
use serde::Deserialize;
use toml::{Config, Environment, File};

/// read config from a file.
pub fn read_toml_from_resources<'de, T: Deserialize<'de>>(prefix: &str) -> NacosResult<T> {
    let mut s = Config::default();
    let default = format!("resources/{}.toml", prefix);
    s.merge(File::with_name(default.as_str()))?;
    // config environment conf file.
    let env = std::env::var("RUN_MODE").unwrap_or(String::from("dev"));
    let file_name = format!("resources/{}-{}.toml", prefix, env);
    s.merge(File::with_name(file_name.as_str()))?;
    // from environment
    s.merge(Environment::with_prefix(prefix))?;
    Ok(s.try_into()?)
}

#[test]
fn test_read() {
    let a = read_toml_from_resources::<NacosConfigProperties>("nacos");
    if let Ok(ref p) = a {
        println!("{:?}", p);
    } else {
        eprintln!("error: {:?}", a.unwrap_err());
    }
}
