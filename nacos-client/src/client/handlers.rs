/// A module to process client-side notifications
pub mod client {}
/// A module to process notification from server-side
pub mod server {
    use nacos_core::error::NacosResult;

    #[tonic::async_trait]
    pub trait ServerRequestHandler {
        /// ServerRequestHandler type to process.
        fn ty(&self) -> String;
        /// A function to process the config from server side
        async fn request_reply(&self, request: String) -> NacosResult<String>;
    }
}
