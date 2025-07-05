use crate::ParseStream;
use quote::quote;
use syn::Fields;
use syn::{Attribute, LitStr};

pub fn parse_general_response_context(
    name: &syn::Ident,
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    let ident = &variant.ident;

    let status_code = variant
        .attrs
        .iter()
        .find_map(|attr| parse_status_code(attr))
        .unwrap_or(quote! { 500 });
    let code = variant
        .attrs
        .iter()
        .find_map(|attr| parse_code_string(attr))
        .unwrap_or_else(|| ident.to_string());

    let pattern = match &variant.fields {
        Fields::Named(_) => {
            panic!("Named fields are not supported in enum variants for response parsing")
        }
        Fields::Unit => quote! { #name::#ident },
        Fields::Unnamed(_) => {
            quote! { #name::#ident(..) }
        }
    };

    let body = match &variant.fields {
        Fields::Unit => quote! { format!("{}", self) },
        Fields::Named(_) => {
            panic!("Named fields are not supported in enum variants for response parsing")
        }
        Fields::Unnamed(_) => {
            quote! { self.to_string() }
        }
    };

    quote! {
        #pattern => {
            axum_error_handler::ErrorResponseContext::builder()
                .status_code(#status_code)
                .code(#code.to_string())
                .message(#body)
                .build()
        }
    }
}

fn parse_status_code(attr: &Attribute) -> Option<proc_macro2::TokenStream> {
    if attr.path().is_ident("status_code") {
        let result = attr.parse_args_with(|input: ParseStream| {
            let fmt: LitStr = input.parse()?;

            let val = fmt.value();

            Ok(quote! { #val.parse().unwrap_or(500) })
        });

        if result.is_err() {
            println!("Error parsing status code");
            return Some(quote! { 500 });
        }

        Some(result.unwrap())
    } else {
        None
    }
}

fn parse_code_string(attr: &Attribute) -> Option<String> {
    if attr.path().is_ident("code") {
        let result = attr.parse_args_with(|input: ParseStream| {
            let fmt: LitStr = input.parse().unwrap();

            Ok(fmt.value())
        });

        if result.is_err() {
            return Some("".to_string());
        }

        Some(result.unwrap())
    } else {
        None
    }
}
