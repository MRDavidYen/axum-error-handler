use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::ParseStream, parse_macro_input, Attribute, DeriveInput, Fields, LitStr};

#[proc_macro_derive(AxumErrorResponse, attributes(status_code, code))]
pub fn derive_axum_error_response(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let variants = if let syn::Data::Enum(data_enum) = input.data {
        data_enum.variants
    } else {
        panic!("AxumErrorResponse can only be derived for enums");
    };

    let match_arms = variants.iter().map(|variant| {
        let ident = &variant.ident;

        let status_code = variant
            .attrs
            .iter()
            .find_map(|attr| parse_status_code(attr))
            .unwrap_or(quote! { axum::http::StatusCode::INTERNAL_SERVER_ERROR });
        let code = variant
            .attrs
            .iter()
            .find_map(|attr| parse_code_string(attr))
            .unwrap_or_else(|| ident.to_string());

        let pattern = match &variant.fields {
            Fields::Unit => quote! { #name::#ident },
            Fields::Named(_) => {
                quote! { #name::#ident { .. } }
            }
            Fields::Unnamed(_) => {
                quote! { #name::#ident(..) }
            }
        };

        let body = match &variant.fields {
            Fields::Unit => quote! { format!("{}", self) },
            Fields::Named(_) => {
                quote! { self.to_string() }
            }
            Fields::Unnamed(_) => {
                quote! { self.to_string() }
            }
        };

        quote! {
            #pattern => {
                let body = #body;
                let json = axum::Json(serde_json::json!({
                    "result": null,
                    "error": {
                        "code": #code,
                        "message": body,
                    }
                }));

                axum::http::Response::builder()
                    .status(#status_code)
                    .header("content-type", "application/json")
                    .body(json.into_response().into_body())
                    .unwrap()
            }
        }
    });

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

fn parse_status_code(attr: &Attribute) -> Option<proc_macro2::TokenStream> {
    if attr.path().is_ident("status_code") {
        let result = attr.parse_args_with(|input: ParseStream| {
            let fmt: LitStr = input.parse()?;

            let val = fmt.value();

            Ok(quote! { axum::http::StatusCode::from_u16(#val.parse().unwrap()).unwrap() })
        });

        if result.is_err() {
            println!("Error parsing status code");
            return Some(quote! { axum::http::StatusCode::INTERNAL_SERVER_ERROR });
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
