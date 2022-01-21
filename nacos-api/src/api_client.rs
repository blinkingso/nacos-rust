/// nacos api executors
/// provides a flexible HttpAgent utility dealing with Nacos Apis.
/// restful api client capable provider
#[allow(dead_code)]
pub mod client {
    use anyhow::Result;
    use itertools::Itertools;
    use reqwest::header::HeaderValue;
    use reqwest::{Client, ClientBuilder, Method, Request, Url};
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;
    use std::fmt::Debug;
    use std::time::Duration;

    pub struct HttpAgent {
        client: Client,
    }

    #[doc = "http client configuration"]
    #[derive(Debug, Serialize, Deserialize)]
    pub struct HttpConfig {
        read_timeout: u64,
        connect_timeout: u64,
        https_only: bool,
        max_retry: u32,
    }

    impl Default for HttpConfig {
        fn default() -> Self {
            HttpConfig {
                read_timeout: 5u64,
                connect_timeout: 2u64,
                https_only: false,
                max_retry: 3,
            }
        }
    }

    impl HttpAgent {
        pub fn build(config: Option<HttpConfig>) -> Self {
            let conf = if let Some(config) = config {
                config
            } else {
                HttpConfig::default()
            };

            Self::new(conf)
        }

        fn new(config: HttpConfig) -> Self {
            let config_ = &config;
            let client = ClientBuilder::new()
                .timeout(Duration::from_secs(config_.read_timeout))
                .connect_timeout(Duration::from_secs(config_.connect_timeout))
                .https_only(config_.https_only)
                .build()
                .expect("http client agent init error");

            HttpAgent { client }
        }

        pub fn get_client(&self) -> Client {
            self.client.clone()
        }

        pub async fn get<'a, 'b, R>(
            &self,
            url: &'a str,
            headers: Option<HashMap<&'a str, &'a str>>,
            params: Option<HashMap<&'b str, &'b str>>,
        ) -> Result<R>
        where
            'b: 'a,
            R: Debug + 'a,
        {
            let client = self.client.clone();
            let mut url = String::from(url);
            if let Some(ref params) = params {
                let query = Query(params);
                if let Some(ref query_string) = query.query() {
                    url.push('?');
                    url.push_str(query_string)
                }
            }
            let mut request = Request::new(Method::GET, Url::parse(url.as_str())?);
            if let Some(ref headers) = headers {
                let mut request_headers = request.headers_mut();
                for header in headers.iter() {
                    request_headers.append(*header.0, HeaderValue::from_str(*header.1)?);
                }
            }

            // try to request
            let resp = client.execute(request).await?;
            todo!()
        }
    }

    struct Query<'a>(&'a HashMap<&'a str, &'a str>);

    impl<'a> Query<'a> {
        pub fn query(&self) -> Option<String> {
            let params = self.0;
            if params.is_empty() {
                None
            } else {
                let append = params
                    .iter()
                    .map(move |kv| {
                        let mut s = String::from(*kv.0);
                        s.push('=');
                        s.push_str(*kv.1);
                        s
                    })
                    .join("&");
                Some(append)
            }
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::Query;
        use std::collections::HashMap;

        #[test]
        fn test_query_params() {
            let mut params = HashMap::<&str, &str>::new();
            params.insert("groupId", "group_id");
            params.insert("dataId", "data_id");
            {
                let query = Query(&params);
                if let Some(query_str) = query.query() {
                    println!("query_str is {}", query_str);
                }
            }
        }
    }
}
