use proc_macro::TokenStream;
use quote::quote;
use syn::{Attribute, DeriveInput, Fields, LitStr, parse::ParseStream, parse_macro_input};

pub(crate) mod response;

#[proc_macro_derive(AxumErrorResponse, attributes(status_code, code, response))]
pub fn derive_axum_error_response(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let syn::Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("AxumErrorResponse can only be derived for enums");
    };

    let match_arms = variants
        .iter()
        .map(|variant| response::parse_response(&name, variant));

    // Generate the final impl block
    let expanded = quote! {
        impl axum::response::IntoResponse for #name {
            fn into_response(self) -> axum::response::Response {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
