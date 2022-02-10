#[macro_export]
macro_rules! impl_internal_request {
   (
       $($target_ty:ty),+ $(,)?
   ) => {
         $(
            impl  crate::api::remote::request::InternalRequest for $target_ty {}
        )*
    }
}

/// impl ServerRequest trait for T
#[macro_export]
macro_rules! impl_server_request {
    (
        $($target_ty:ty),+ $(,)?
    ) => {
         $(
            impl crate::api::remote::request::ServerRequest for $target_ty {}
        )*
    }
}

/// impl ConfigRequest trait for T
#[macro_export]
macro_rules! impl_config_request {
    (
        $($target_ty:ty),+ $(,)?
    ) => {
         $(
            impl crate::api::remote::request::ConfigRequest for $target_ty {}
        )*
    }
}

#[macro_export]
macro_rules! impl_req_ext {
    (
        $($target:ty),+ $(,)?
    ) => {
        $(
            impl ::std::ops::Deref for $target {
                type Target = crate::api::remote::request::RpcRequest;
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

        impl ::std::ops::DerefMut for $target {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.inner
                }
            }

        impl crate::api::traits::RequestExt for $target {
                fn ty_name(&self) -> String {
                    String::from(stringify!($target))
                }

                fn headers(&self) -> HashMap<String, String> {
                    self.inner.headers()
                }

                fn clear_headers(&mut self) {
                    self.inner.clear_headers()
                }
        }
         )+
    };
}

#[macro_export]
macro_rules! impl_resp_ext {
    (
        $($target:ty),+ $(,)?
    ) => {
        $(
            impl ::std::ops::Deref for $target {
                type Target = crate::api::remote::response::RpcResponse;
                fn deref(&self) -> &Self::Target {
                    &self.inner
                }
            }

            impl ::std::ops::DerefMut for $target {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.inner
                    }
                }
            impl crate::api::traits::ResponseExt for $target {
                    fn ty_name(&self) -> String {
                        String::from(stringify!($target))
                    }
            }
        )+
    };
}
