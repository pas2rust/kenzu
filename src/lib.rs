use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
mod components;
use components::prelude::*;

#[proc_macro_derive(Builder, attributes(opt))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let generate = generate(&input);
    generate.into()
}
