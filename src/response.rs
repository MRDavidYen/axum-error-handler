use core::panic;

use quote::quote;
use syn::{Attribute, Ident, Variant, parse::ParseStream};

use crate::response::{general::parse_general_response, nested::parse_nested_response};

pub(crate) mod general;
pub(crate) mod nested;

pub(super) enum ResponseType {
    Nested,
    General,
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
                return parse_nested_response(parent_name, variant);
            } else if ident == "general" {
                return parse_general_response(parent_name, variant);
            } else {
                panic!("Unknown response type: {}", ident);
            }
        }
    }

    // Default to general response parsing if no specific response type is found
    parse_general_response(parent_name, variant)
}
