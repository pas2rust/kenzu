use super::prelude::*;

pub fn generate_method(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_ident: &Ident = field.ident.as_ref().expect("field name must be set");
    let Opt {
        name,
        err,
        pattern,
        default,
        err_max,
        err_min,
        max,
        min,
    } = get_opt(&field.attrs);
    let ty = &field.ty;
    let struct_name = get_struct_name(input);
    let (impl_generics, type_generics, where_clause) = generics_split_for_impl(input);
    let type_name_ts: TokenStream = get_type_name_ts(&struct_name, field_ident, name);

    if is_string(ty) || is_str_ref(ty) {
        let re_ident = Ident::new(
            &format!("RE_{}_{}", struct_name, field_ident),
            Span::call_site(),
        );
        let get_re_fn = Ident::new(
            &format!("get_re_{}_{}", struct_name, field_ident),
            Span::call_site(),
        );

        let pattern_ts: TokenStream = match pattern {
            Some(ts) => quote! { #ts },
            None => quote! { ".*" },
        };
        let err_ts: TokenStream = match err {
            Some(ts) => quote! { #ts },
            None => {
                let type_name_str = type_name_ts.to_string();
                quote! { concat!("Regex pattern invalid for ", #type_name_str) }
            }
        };

        let default_expr_tokens = if let Some(d) = default {
            quote! { #d }
        } else {
            let struct_ident = struct_name;
            quote! { #struct_ident::default().#field_ident }
        };

        quote! {
            static #re_ident: ::std::sync::OnceLock<::regex::Regex> = ::std::sync::OnceLock::new();
            fn #get_re_fn() -> &'static ::regex::Regex {
                #re_ident.get_or_init(|| {
                    ::regex::Regex::new(#pattern_ts)
                        .expect("invalid regex literal")
                })
            }

            impl ::std::str::FromStr for #type_name_ts {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    if #get_re_fn().is_match(s) {
                        Ok(#type_name_ts(s.to_string()))
                    } else {
                        Err(#err_ts.to_string())
                    }
                }
            }

            impl #type_name_ts {
                pub fn default() -> Self {
                    let s: &str = &#default_expr_tokens;
                    ::std::str::FromStr::from_str(s)
                        .expect("invalid default value for field")
                }

                pub fn new<S: AsRef<str>>(s: S) -> Result<Self, String> {
                    ::std::str::FromStr::from_str(s.as_ref())
                }
            }
        }
    } else if is_numeric(ty) {
        let err_msg_ts = match err {
            Some(ts) => quote! { #ts },
            None => {
                let type_name_str = type_name_ts.to_string();
                quote! { concat!("Invalid number for ", #type_name_str) }
            }
        };
        let err_min_ts = match err_min {
            Some(ts) => quote! { #ts },
            None => {
                let type_name_str = type_name_ts.to_string();
                quote! { concat!("Value for ", #type_name_str, " is less than minimum") }
            }
        };
        let err_max_ts = match err_max {
            Some(ts) => quote! { #ts },
            None => {
                let type_name_str = type_name_ts.to_string();
                quote! { concat!("Value for ", #type_name_str, " is greater than maximum") }
            }
        };

        let default_expr_tokens = if let Some(d) = default {
            quote! { #d }
        } else {
            let struct_ident = struct_name;
            quote! { #struct_ident::default().#field_ident }
        };

        let min_check = if let Some(min_ts) = &min {
            quote! {
                if v < (#min_ts) {
                    return Err(#err_min_ts.to_string());
                }
            }
        } else {
            quote! {}
        };

        let max_check = if let Some(max_ts) = &max {
            quote! {
                if v > (#max_ts) {
                    return Err(#err_max_ts.to_string());
                }
            }
        } else {
            quote! {}
        };

        let default_min_check = if let Some(min_ts) = &min {
            quote! {
                if v < (#min_ts) {
                    panic!("{}: {}", #err_min_ts, #min_ts);
                }
            }
        } else {
            quote! {}
        };

        let default_max_check = if let Some(max_ts) = &max {
            quote! {
                if v > (#max_ts) {
                    panic!("{}: {}", #err_max_ts, #max_ts);
                }
            }
        } else {
            quote! {}
        };

        quote! {
            impl ::std::str::FromStr for #type_name_ts {
                type Err = String;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    let v: #ty = s.parse().map_err(|e| format!("{}({})", #err_msg_ts, e))?;
                    #min_check
                    #max_check
                    Ok(#type_name_ts(v))
                }
            }

            impl #type_name_ts {
                pub fn default() -> Self {
                    let v: #ty = #default_expr_tokens;
                    #default_min_check
                    #default_max_check
                    #type_name_ts(v)
                }

                pub fn new<V: Into<#ty>>(value: V) -> Result<Self, String> {
                    let v: #ty = value.into();
                    #min_check
                    #max_check
                    Ok(#type_name_ts(v))
                }
            }
        }
    } else {
        let default_expr_tokens = if let Some(d) = default {
            quote! { #d }
        } else {
            let struct_ident = struct_name;
            quote! { #struct_ident::default().#field_ident }
        };
        quote! {
            impl #impl_generics #type_name_ts #type_generics #where_clause {
                pub fn default() -> Self {
                    let v: #ty = #default_expr_tokens;
                    #type_name_ts(v)
                }

                pub fn new(v: #ty) -> Result<Self, String> {
                    Ok(#type_name_ts(v))
                }
            }
        }
    }
}
