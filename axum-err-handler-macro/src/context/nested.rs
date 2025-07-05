use quote::quote;

pub fn parse_nested_response_context(
    name: &syn::Ident,
    variant: &syn::Variant,
) -> proc_macro2::TokenStream {
    // match_pattern is will generate like: `EnumName::VariantName(inner)`
    let match_pattern = match &variant.fields {
        syn::Fields::Named(_) => {
            panic!("Named fields are not supported in enum variants for nested response parsing")
        }
        syn::Fields::Unit => quote! { panic!(
            "there is no inner value that implement `IntoResponse` trait",
        ) },
        syn::Fields::Unnamed(_) => {
            let variant_ident = &variant.ident;
            quote! { #name::#variant_ident(inner) }
        }
    };

    let extract = match &variant.fields {
        syn::Fields::Unit => quote! { panic!(
            "there is no inner value that implement `IntoResponse` trait",
        ) },
        syn::Fields::Named(_) => {
            panic!("Named fields are not supported in enum variants for nested response parsing")
        }
        syn::Fields::Unnamed(_) => {
            quote! { inner.into_response_context() }
        }
    };

    quote! {
        #match_pattern => {
            #extract
        }
    }
}
