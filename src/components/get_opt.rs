use super::prelude::*;

#[derive(Debug, Default)]
pub struct Opt {
    pub name: Option<TokenStream>,
    pub pattern: Option<TokenStream>,
    pub err: Option<TokenStream>,
    pub default: Option<TokenStream>,
    pub min: Option<TokenStream>,
    pub max: Option<TokenStream>,
    pub err_min: Option<TokenStream>,
    pub err_max: Option<TokenStream>,
}

pub fn get_opt(attributes: &Vec<Attribute>) -> Opt {
    let mut opt = Opt::default();

    for attr in attributes {
        if attr.path().is_ident("opt") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    if let Ok(ts) = meta.value()?.parse::<proc_macro2::TokenStream>() {
                        opt.name = Some(ts);
                    }
                } else if meta.path.is_ident("pattern") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.pattern = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("err") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.err = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("default") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.default = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("min") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.min = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("max") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.max = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("err_min") {
                    if let Ok(expr) = meta.value()?.parse::<syn::Expr>() {
                        opt.err_min = Some(quote! { #expr });
                    }
                } else if meta.path.is_ident("err_max")
                    && let Ok(expr) = meta.value()?.parse::<syn::Expr>()
                {
                    opt.err_max = Some(quote! { #expr });
                }

                Ok(())
            })
            .unwrap();
        }
    }

    opt
}
