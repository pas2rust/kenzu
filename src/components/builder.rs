use super::prelude::*;
use mokuya::components::prelude::*;
use quote::quote;
use syn::DeriveInput;

pub fn generate_build(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;

    let impl_block_name = get_impl(input);

    let default_init = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field without identifier.");
        let field_type = &field.ty;

        let SetValue { value, .. } = get_set_value(&field.attrs);

        if let Some(value) = value {
            quote! {
                #field_name: #value.into()
            }
        } else {
            quote! {
                #field_name: <#field_type as Default>::default()
            }
        }
    });

    let methods = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field without identifier.");
        let field_type = &field.ty;
        let is_async = is_async_fn(field_type);

        if is_async {
            quote! {
                pub async fn #field_name<New: Into<#field_type> + std::fmt::Debug>(mut self, new: New) -> Self {
                    self.#field_name = new.into().await;
                    self
                }
            }
        } else {
            quote! {
                pub fn #field_name<New: Into<#field_type> + std::fmt::Debug>(mut self, new: New) -> Self {
                    self.#field_name = new.into();
                    self
                }
            }
        }
    });

    let check_pattern = fields.iter().map(validator_pattern);
    let check_range = fields.iter().map(|field| validator_range(input, field));

    let async_fields_init = fields.iter().map(|field| {
        let field_name = field.ident.as_ref().expect("Field without identifier.");
        let field_type = &field.ty;
        let is_async = is_async_fn(field_type);

        if is_async {
            quote! {
                #field_name: instance.#field_name.await
            }
        } else {
            quote! {
                #field_name: instance.#field_name
            }
        }
    });

    let impl_block = quote! {
        // #[cfg_attr(feature = "tracing", mdd::debugger_impl)]
        impl #impl_block_name {
            pub fn new() -> Self {
                Self {
                    #(#default_init),*
                }
            }

            pub fn build(self) -> Result<Self, String> {
                #(#check_pattern)*
                #(#check_range)*
                Ok(self)
            }

            pub async fn async_new() -> Self {
                let instance = Self::new();
                Self {
                    #(#async_fields_init),*
                }
            }

            #(#methods)*
        }
    };

    impl_block
}
