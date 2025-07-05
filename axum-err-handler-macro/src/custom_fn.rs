use syn::{DeriveInput, LitStr};

/// Parses the `response` attribute from the derive input to find a custom function name.
pub fn parse_custom_fn(input: &DeriveInput) -> Option<LitStr> {
    let custom_fn = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("response"));

    if let Some(custom_fn) = custom_fn {
        let result = custom_fn.parse_args_with(|input: syn::parse::ParseStream| {
            let ident: syn::Ident = input.parse()?;

            if ident != "custom_fn" {
                return Err(syn::Error::new(ident.span(), "Expected 'custom_fn'"));
            }

            // Parse the '=' token
            let _: syn::Token![=] = input.parse()?;

            // Parse the string literal value
            let lit_str: syn::LitStr = input.parse()?;

            Ok(lit_str)
        });

        match result {
            Ok(function_name) => Some(function_name),
            Err(_) => None,
        }
    } else {
        None
    }
}
