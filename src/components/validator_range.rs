use super::prelude::*;
use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, Lit};

pub fn validator_range(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().expect("field_name must be set");
    let attributes = get_attributes(input);
    let range = get_range(attributes);

    fn try_parse_literal(ts: &TokenStream) -> Option<Lit> {
        syn::parse2::<Lit>(ts.clone()).ok()
    }

    if let (Some(min_ts), Some(max_ts)) = (&range.min, &range.max) {
        if let (Some(min_lit), Some(max_lit)) = (try_parse_literal(min_ts), try_parse_literal(max_ts)) {
            let min_val_opt = match &min_lit {
                Lit::Int(lit_int) => lit_int.base10_parse::<i128>().ok(),
                Lit::Float(lit_float) => lit_float.base10_parse::<f64>().ok().map(|f| f as i128),
                _ => None,
            };
            let max_val_opt = match &max_lit {
                Lit::Int(lit_int) => lit_int.base10_parse::<i128>().ok(),
                Lit::Float(lit_float) => lit_float.base10_parse::<f64>().ok().map(|f| f as i128),
                _ => None,
            };

            if let (Some(min_val), Some(max_val)) = (min_val_opt, max_val_opt) {
                if min_val > max_val {
                    return syn::Error::new_spanned(
                        field,
                        format!("invalid range: min ({}) cannot be greater than max ({})", min_val, max_val),
                    )
                    .to_compile_error();
                }
            }
        }
    }

    let mut validation_code = TokenStream::new();

    if let Some(min_ts) = &range.min {
        if let Some(min_error) = &range.err_min {
            validation_code.extend(quote! {
                if self.#field_name < #min_ts {
                    return Err(#min_error);
                }
            });
        }
    }

    if let Some(max_ts) = &range.max {
        if let Some(max_error) = &range.err_max {
            validation_code.extend(quote! {
                if self.#field_name > #max_ts {
                    return Err(#max_error);
                }
            });
        }
    }

    validation_code
}
