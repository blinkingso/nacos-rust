use serde::Deserialize;
use std::fmt::Formatter;

/// ConfigType
#[derive(Debug, Deserialize)]
pub struct ConfigType<'a>(&'a str);

macro_rules! config_type {
    (
        $(
            $(#[$docs:meta])*
            ($type:ident, $value:expr);
        )+
    ) => {
        impl<'a> ConfigType<'a> {
            // const types here.
            $(
                $(#[$docs])*
                pub const $type: ConfigType<'static> = ConfigType($value);
            )+

            fn config_type(&self) -> String {
                self.0.to_string()
            }
        }

        impl<'a> From<&str> for ConfigType<'a> {
            fn from(value: &str) -> Self {
                match value {
                    // "text" => ConfigType::TEXT,
                    // "json" => ConfigType::JSON,
                    // "html" => ConfigType::HTML,
                    // "properties" => ConfigType::PROPERTIES,
                    // "yaml" => ConfigType::YAML,
                    // "xml" => ConfigType::XML,
                    // others => panic!("Unsupported config type: {}", others),
                    $(
                        $value => ConfigType::$type,
                    )+
                    others => panic!("Unsupported config type: {}", others),
                }
            }
        }
    };
}

config_type! {
    #[doc = "type of properties"]
    (PROPERTIES, "properties");
    #[doc = "type of xml"]
    (XML, "xml");
    #[doc = "type of json"]
    (JSON, "json");
    #[doc = "type of text"]
    (TEXT, "text");
    #[doc = "type of html, now not supported"]
    (HTML, "html");
    #[doc = "type of yaml"]
    (YAML, "yaml");
}

impl<'a> std::fmt::Display for ConfigType<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_config_type() {
    let t = ConfigType::XML;
    println!("type = {}", t);
    let s = ConfigType::from("json");
    println!("type = {}", s);
}
