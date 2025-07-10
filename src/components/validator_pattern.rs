use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use syn::Field;

use crate::components::get_build::{Builder, get_build};

pub fn validator_pattern(field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().expect("field_name must be set");
    let Builder { err, pattern } = get_build(&field.attrs);

    if let (Some(pattern), Some(err)) = (pattern.as_ref(), err.as_ref()) {
        if let Err(e) = Regex::new(pattern) {
            return syn::Error::new_spanned(field, format!("invalid regex pattern: {}", e))
                .to_compile_error();
        }

        return quote! {
            {
                static RE: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
                    regex::Regex::new(#pattern).expect("Failed to compile regex")
                });

                if !RE.is_match(&self.#field_name) {
                    return Err(#err.into());
                }
            }
        };
    }

    quote! {}
}
