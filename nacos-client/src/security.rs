use chrono::Utc;
use log::{debug, info, warn};
use nacos_common::error::{NacosError, NacosResult};
use serde::Deserialize;
use std::sync::{Arc, Mutex};

pub const LOGIN_URL: &'static str = "/v1/auth/users/login";
pub const HTTP_PREFIX: &'static str = "http";
pub const HTTPS_PREFIX: &'static str = "https";

#[derive(Debug, Clone)]
pub struct Credentials {
    pub(crate) username: Option<String>,
    pub(crate) password: Option<String>,
}

impl Credentials {
    fn enabled(&self) -> bool {
        self.username.is_some() && !self.username.as_ref().unwrap().trim().is_empty()
    }
}

#[derive(Debug)]
pub struct SecurityProxy {
    // if credentials is none, no auth needed here.
    credentials: Option<Credentials>,
    access_token: String,
    context_path: String,
    token_ttl: i64,
    last_refresh_time: i64,
    token_refresh_window: i64,
}

impl Default for SecurityProxy {
    fn default() -> Self {
        SecurityProxy {
            credentials: None,
            access_token: "".to_string(),
            context_path: "".to_string(),
            token_ttl: 0,
            last_refresh_time: 0,
            token_refresh_window: 0,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    token_ttl: i64,
    global_admin: bool,
    access_token: String,
}

pub async fn login(
    credentials: Credentials,
    server_url: &str,
    context_path: &str,
) -> NacosResult<SecurityProxy> {
    if !credentials.enabled() {
        warn!("no authentication message found, please check to auth.");
        return Ok(SecurityProxy::default());
    };
    let url = format!(
        "{}://{}{}{}",
        HTTP_PREFIX, server_url, context_path, LOGIN_URL
    );
    let params = [("username", credentials.username.as_ref().unwrap().as_str())];
    let body = [("password", credentials.password.as_ref().unwrap().as_str())];
    let resp = crate::http::post_form(url, &params, &body).await?;
    debug!("response string is : {}", resp);
    let result = serde_json::from_str::<LoginResponse>(resp.as_str())?;
    let access_token = result.access_token.clone();
    let token_ttl = result.token_ttl;
    let token_refresh_window = token_ttl / 10;
    Ok(SecurityProxy {
        credentials: Some(credentials.clone()),
        access_token,
        context_path: context_path.to_string(),
        token_ttl,
        last_refresh_time: Utc::now().timestamp_millis(),
        token_refresh_window,
    })
}

pub async fn refresh_login(
    security_proxy: Arc<Mutex<SecurityProxy>>,
    server_urls: &Vec<String>,
) -> NacosResult<()> {
    // lock to check login state.
    let sp = security_proxy.clone();
    {
        let lock = sp.lock().unwrap();
        let now = Utc::now().timestamp_millis();
        if now - lock.last_refresh_time < lock.token_ttl - lock.token_refresh_window {
            return Ok(());
        }
    }

    let sp = security_proxy.clone();
    let (credentials, context_path) = {
        let lock = sp.lock().unwrap();
        (
            lock.credentials
                .as_ref()
                .ok_or(NacosError::msg("no credentials specified."))?
                .clone(),
            lock.context_path.as_str().to_string(),
        )
    };

    // try to login here.
    for server in server_urls {
        let sp = security_proxy.clone();
        match login(credentials.clone(), server, context_path.clone().as_str()).await {
            Ok(_sp) => {
                info!("{:?}", _sp);
                let mut lock = sp.lock().unwrap();
                *lock = _sp;
                return Ok(());
            }
            Err(err) => {
                warn!("login failed, {:?}", err);
                continue;
            }
        }
    }

    // none server is in login.
    Err(NacosError::msg(format!(
        "none server login success, server list: {:?}",
        server_urls
    )))
}

#[test]
fn test_req() {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::init();
    let test = async {
        let credentials = Credentials {
            username: Some("nacos".to_string()),
            password: Some("nacos".to_string()),
        };
        let mut security = SecurityProxy {
            credentials: Some(credentials),
            access_token: "".to_string(),
            context_path: "/nacos".to_string(),
            token_ttl: 0,
            last_refresh_time: 0,
            token_refresh_window: 0,
        };
        let sp = Arc::new(Mutex::new(security));
        let res = refresh_login(sp.clone(), &["127.0.0.1:8848".to_string()].to_vec()).await;
        if res.is_ok() {
            Ok(sp)
        } else {
            Err(NacosError::msg("login error."))
        }
    };

    let result = tokio::runtime::Runtime::new().unwrap().block_on(test);
    match result {
        Ok(flag) => {
            info!("login success: {:?}", flag.lock().unwrap());
        }
        Err(e) => {
            warn!("login error for: {:?}", e);
        }
    }
}
