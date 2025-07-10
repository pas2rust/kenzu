use proc_macro2::TokenStream;
use syn::{Attribute, LitStr};

#[derive(Debug, Default)]
pub struct Range {
    pub min: Option<TokenStream>,
    pub max: Option<TokenStream>,
    pub err_min: Option<String>,
    pub err_max: Option<String>,
}

pub fn get_range(attributes: Vec<Attribute>) -> Range {
    let mut range = Range::default();

    for attr in attributes {
        if attr.path().is_ident("range") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("min") {
                    if let Ok(token) = meta.value()?.parse::<TokenStream>() {
                        range.min = Some(token);
                    }
                } else if meta.path.is_ident("max") {
                    if let Ok(token) = meta.value()?.parse::<TokenStream>() {
                        range.max = Some(token);
                    }
                } else if meta.path.is_ident("err_max") {
                    if let Ok(err) = meta.value()?.parse::<LitStr>() {
                        range.err_max = Some(err.value());
                    }
                } else if meta.path.is_ident("err_min") {
                    if let Ok(err) = meta.value()?.parse::<LitStr>() {
                        range.err_min = Some(err.value());
                    }
                }
                Ok(())
            })
            .unwrap();
        }
    }

    range
}
