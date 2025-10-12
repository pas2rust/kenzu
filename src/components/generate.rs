use super::prelude::*;

pub fn generate(input: &DeriveInput) -> proc_macro2::TokenStream {
    let input_clone = input.clone();
    let fields = &get_named_fields(&input_clone)
        .expect("Failed to get fields: ensure the struct is valid.")
        .named;
    let impl_block = get_impl(input);
    let struct_name = get_struct_name(input);
    let field_methods: Vec<_> = fields
        .iter()
        .map(|f| generate_field_methods(&input, f))
        .collect();
    let new_fields: Vec<_> = fields
        .iter()
        .map(|f| {
            let field_ident = f.ident.as_ref().unwrap();
            let type_ident = get_type_name_ts(&struct_name, field_ident, get_opt(&f.attrs).name);
            quote! { #field_ident: #type_ident::default().into() }
        })
        .collect();
    let field_defaults: Vec<(Ident, Option<TokenStream>)> = fields
        .iter()
        .map(|f| {
            let ident = f.ident.as_ref().unwrap().clone();
            let opt = get_opt(&f.attrs);
            let default_ts = opt.default.clone();
            (ident, default_ts)
        })
        .collect();

    let default_impl = generate_default_impl(&struct_name, &field_defaults);

    quote! {
        #(#field_methods)*

        impl #impl_block {
            pub fn new() -> Self {
                Self {
                    #(#new_fields),*
                }
            }
        }

        #default_impl
    }
}

pub fn generate_default_impl(
    struct_ident: &Ident,
    field_defaults: &[(Ident, Option<TokenStream>)],
) -> TokenStream {
    let default_inits = field_defaults
        .iter()
        .map(|(ident, default_opt)| match default_opt {
            Some(ts) => quote! { #ident: #ts.into(), },
            None => quote! { #ident: ::std::default::Default::default(), },
        });
    quote! {
        impl ::std::default::Default for #struct_ident {
            fn default() -> Self {
                Self {
                    #(#default_inits)*
                }
            }
        }
    }
}
