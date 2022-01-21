use nacos_common::error::{NacosError, NacosResult};
use log::{debug, error};
use reqwest::ClientBuilder;
use std::time::Duration;
pub async fn post_form<'a>(
    url: String,
    params: &'a [(&'static str, &'a str)],
    body: &'a [(&'static str, &'a str)],
) -> NacosResult<String> {
    let client = ClientBuilder::new()
        .https_only(false)
        .timeout(Duration::from_secs(15))
        .connect_timeout(Duration::from_secs(5))
        .no_proxy()
        .gzip(true)
        .build()
        .unwrap();
    debug!("request url : {}", &url);
    let response = client.post(&url).query(&params).form(&body).send().await;
    match response {
        Ok(resp) => {
            let code = resp.status();
            if code.is_success() {
                Ok(resp.text_with_charset("UTF-8").await?)
            } else {
                Err(NacosError::msg(format!(
                    "request to {} error for: {}",
                    &url,
                    code.canonical_reason().unwrap_or("unknown error.")
                )))
            }
        }
        Err(e) => {
            error!("http response error: {:?}", e);
            return Err(NacosError::new(e));
        }
    }
}
