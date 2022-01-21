extern crate proc_macro;
extern crate proc_macro_error;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::{abort_call_site, proc_macro_error, set_dummy};
use quote::{quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{
    Attribute, DataStruct, DeriveInput, Field, Generics, ImplGenerics, Path, TypeGenerics,
    TypeParamBound, WhereClause,
};

#[proc_macro_derive(NacosConfig, attributes(NacosValue))]
#[proc_macro_error]
pub fn nacos_config_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse(input).unwrap();
    let gen = impl_nacos_config(&input);
    gen.into()
}

/// check `NacosValue` values is format.
/// Examples
/// ```
/// #[NacosConfig(dataId = "data_id", groupId = "group", autoRefreshed = true)]
/// pub struct Config {
///     #[NacosValue(value = "", autoRefreshed = true)]
///     url: String,
///     message: String,
/// }
/// ```
fn check_nacos_value_format<T: ToTokens>(to_tokens: T) -> Option<proc_macro2::TokenStream> {
    None
}

fn impl_nacos_config(input: &DeriveInput) -> TokenStream {
    use syn::Data::*;
    let struct_name = &input.ident;
    set_dummy(quote! {
        impl ::nacos_common::NacosConfig for #struct_name {
            fn load() -> Self where Self: Sized {
                unimplemented!()
            }

            fn refresh(&mut self) {
                unimplemented!()
            }

            fn notify(&mut self, configuration: String) {
                unimplemented!()
            }

            fn is_auto_refreshed(&self) -> bool {
                true
            }

            fn get_interval_secs(&self) -> u32 {
                30 * 60
            }
        }
    });

    match input.data {
        Struct(DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) => {
            let generics = &input.generics;
            impl_nacos_config_for_struct(struct_name, &fields.named, &input.attrs, &input.generics)
        }

        _ => abort_call_site!("NacosConfig only supports non-tuple Structs."),
    }
}

fn impl_nacos_config_for_struct(
    name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
    generics: &Generics,
) -> TokenStream {
    // let (impl_generics, ty_generics, where_clause) =
    //     split_nacos_config_generics_for_impl(&generics);
    todo!()
}

fn split_nacos_config_generics_for_impl(
    generics: &Generics,
) -> (ImplGenerics, TypeGenerics, TokenStream) {
    use syn::{token::Add, TypeParamBound::Trait};

    fn path_ends_with(path: &Path, ident: &str) -> bool {
        path.segments.last().unwrap().ident == ident
    }

    fn type_param_bounds_contains(bounds: &Punctuated<TypeParamBound, Add>, ident: &str) -> bool {
        for bound in bounds {
            if let Trait(bound) = bound {
                if path_ends_with(&bound.path, ident) {
                    return true;
                }
            }
        }

        return false;
    }

    struct TraitBoundAmendments {
        tokens: TokenStream,
        need_where: bool,
        need_comma: bool,
    }

    impl TraitBoundAmendments {
        fn new(where_clause: Option<&WhereClause>) -> Self {
            let tokens = TokenStream::new();
            let (need_where, need_comma) = if let Some(where_clause) = where_clause {
                if where_clause.predicates.trailing_punct() {
                    (false, false)
                } else {
                    (false, true)
                }
            } else {
                (true, false)
            };
            Self {
                tokens,
                need_where,
                need_comma,
            }
        }

        fn add(&mut self, amendment: TokenStream) {
            if self.need_where {
                self.tokens.extend(quote! {where});
                self.need_where = false;
            }
            if self.need_comma {
                self.tokens.extend(quote! {,});
            }
            self.tokens.extend(amendment);
            self.need_comma = true;
        }

        fn into_tokens(self) -> TokenStream {
            self.tokens
        }
    }

    // let mut trait_bound_amendments = TraitBoundAmendments::new(generics.where_clause.as_ref());
    // for param in &generics.params {
    //     if let GenericParam::Type(param) = param {
    //         let param_ident = &param.ident;
    //         if type_param_bounds_contains(&param.bounds, "NacosConfig") {
    //             trait_bound_amendments
    //         }
    //     }
    // }
    todo!()
}
