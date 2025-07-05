use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse::ParseStream, parse_macro_input};

use crate::context::parse_final_response_context_block;

pub(crate) mod context;
mod custom_fn;

#[proc_macro_derive(AxumErrorResponse, attributes(status_code, code, response))]
pub fn derive_axum_error_response(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let custom_fn = custom_fn::parse_custom_fn(&input);
    let name = input.ident.clone();

    let response_block = parse_final_response_context_block(&name, &input);

    let mut expand = quote! {
        #response_block
    };

    if custom_fn.is_some() {
        let custom_fn_name = custom_fn.unwrap();

        let fn_name = custom_fn_name.value();
        let fn_ident = syn::Ident::new(&fn_name, custom_fn_name.span());

        expand.extend(quote! {
            impl axum::response::IntoResponse for #name {
                fn into_response(self) -> axum::response::Response {
                    use axum_error_handler::IntoErrorResponseContext;

                    #fn_ident(self.into_response_context())
                }
            }
        });

        TokenStream::from(expand)
    } else {
        expand.extend(quote! {
            impl axum::response::IntoResponse for #name {
                fn into_response(self) -> axum::response::Response {
                    use axum_error_handler::IntoErrorResponseContext;

                    self.into_response_context().into_response()
                }
            }
        });

        TokenStream::from(expand)
    }
}
