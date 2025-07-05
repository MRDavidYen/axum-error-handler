use core::panic;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Ident, Variant, parse::ParseStream};

use crate::context::{
    general::parse_general_response_context, nested::parse_nested_response_context,
};

mod general;
mod nested;

pub fn parse_final_response_context_block(
    name: &Ident,
    input: &DeriveInput,
) -> proc_macro2::TokenStream {
    let variants = if let syn::Data::Enum(data_enum) = &input.data {
        data_enum.variants.clone()
    } else {
        panic!("AxumErrorResponse can only be derived for enums");
    };

    let match_arms = variants
        .iter()
        .map(|variant| parse_response(&name, variant));

    // Generate the final impl block
    let expanded = quote! {
        impl axum_error_handler::IntoErrorResponseContext for #name {
            fn into_response_context(self) -> axum_error_handler::ErrorResponseContext {
                match self {
                    #(#match_arms),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}

/// Parse `response` attribute from a variant to determine if it's nested.
pub fn parse_response(parent_name: &syn::Ident, variant: &Variant) -> proc_macro2::TokenStream {
    if let Some(response_attr) = variant
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("response"))
    {
        let result = response_attr.parse_args_with(|input: ParseStream| {
            let ident: Ident = input.parse()?;
            Ok(ident)
        });

        if let Ok(ident) = result {
            if ident == "nested" {
                return parse_nested_response_context(parent_name, variant);
            } else if ident == "general" {
                return parse_general_response_context(parent_name, variant);
            } else {
                panic!("Unknown response type: {}", ident);
            }
        }
    }

    // Default to general response parsing if no specific response type is found
    parse_general_response_context(parent_name, variant)
}
