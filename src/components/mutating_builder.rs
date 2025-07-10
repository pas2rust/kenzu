use super::prelude::*;
use mokuya::components::prelude::*;
use quote::quote;
use syn::DeriveInput;

pub fn generate_mutating_build(input: &DeriveInput) -> proc_macro2::TokenStream {
    let impl_block = get_impl(input);
    let fields = get_named_fields(input)
        .expect("Failed to get fields. Ensure the struct is valid.")
        .named
        .iter();

    // === Generate setter methods ===
    let setters = fields.clone().filter_map(|field| {
        let field_name = field.ident.as_ref().expect("Unnamed field.");
        let field_type = &field.ty;

        let set_value = super::get_set::get_set_value(&field.attrs);
        if set_value.skip {
            return None; // Skip generating setter if #[set(skip)]
        }

        Some(if is_async_fn(field_type) {
            quote! {
                pub async fn #field_name<Darth: Into<#field_type>>(&mut self, new: Darth) -> &mut Self {
                    self.#field_name = new.into().await;
                    self
                }
            }
        } else {
            quote! {
                pub fn #field_name<Darth: Into<#field_type>>(&mut self, new: Darth) -> &mut Self {
                    self.#field_name = new.into();
                    self
                }
            }
        })
    });

    // === Validation checks ===
    let pattern_checks = fields.clone().map(|field| validator_pattern(field));
    let range_checks = fields.clone().map(|field| validator_range(input, field));

    // === Async initializer ===
    let async_init = fields.clone().map(|field| {
        let field_name = field.ident.as_ref().expect("Unnamed field.");
        let field_type = &field.ty;

        if is_async_fn(field_type) {
            quote! {
                #field_name: instance.#field_name.await
            }
        } else {
            quote! {
                #field_name: instance.#field_name
            }
        }
    });

    // === Default initialization ===
    let defaults = fields.map(|field| {
        let field_name = field.ident.as_ref().expect("Unnamed field.");
        let field_type = &field.ty;
        let field_attr = &field.attrs;

        let set_value = super::get_set::get_set_value(&field_attr);

        if let Some(value) = set_value.value {
            quote! {
                #field_name: #value.into()
            }
        } else {
            quote! {
                #field_name: <#field_type as Default>::default()
            }
        }
    });

    // === Final token stream ===
    quote! {
        impl #impl_block {
            pub fn new() -> Self {
                Self {
                    #(#defaults),*
                }
            }

            pub fn build(&mut self) -> Result<&mut Self, String> {
                #(#pattern_checks)*
                #(#range_checks)*
                Ok(self)
            }

            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    #(#async_init),*
                }
            }

            #(#setters)*
        }
    }
}
