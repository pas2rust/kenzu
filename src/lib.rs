use mokuya::components::prelude::add_traits_to_generics;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
mod components;
use components::prelude::*;

//#[cfg(feature = "builder")]
#[proc_macro_derive(Builder, attributes(range, set, build, skip_trait))]
pub fn builder(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);
    let generate_build = generate_build(&input);
    generate_build.into()
}

//#[cfg(feature = "mutating_builder")]
#[proc_macro_derive(M_Builder, attributes(range, set, build, skip_trait))]
pub fn mutating_builder(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);
    let generate_build = generate_mutating_build(&input);
    generate_build.into()
}
