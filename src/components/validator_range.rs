use super::prelude::*;
use mokuya::components::prelude::*;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field};

pub fn validator_range(input: &DeriveInput, field: &Field) -> TokenStream {
    let field_name = field.ident.as_ref().expect("field_name must be set");
    let attributes = get_attributes(input);
    let range = get_range(attributes);

    let mut validation_code = TokenStream::new();

    // Check for min and max value and error message.
    if let (Some(min_ts), Some(max_ts)) = (&range.min, &range.max) {
        if let (Some(min_error), Some(_)) = (&range.err_min, &range.err_max) {
            // Add a runtime check to ensure min is not greater than max.
            validation_code.extend(quote! {
                if #min_ts > #max_ts {
                    return Err(#min_error); // Or a custom error for range
                }
            });
        }
    }

    if let Some(min_ts) = &range.min {
        if let Some(min_error) = &range.err_min {
            validation_code.extend(quote! {
                if self.#field_name < #min_ts {
                    return Err(#min_error.into());
                }
            });
        }
    }

    if let Some(max_ts) = &range.max {
        if let Some(max_error) = &range.err_max {
            validation_code.extend(quote! {
                if self.#field_name > #max_ts {
                    return Err(#max_error.into());
                }
            });
        }
    }

    validation_code
}