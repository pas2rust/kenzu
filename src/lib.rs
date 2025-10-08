use mokuya::components::prelude::add_traits_to_generics;
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};
mod components;
use components::prelude::*;

/// Derive macro `Builder` generates an **immutable builder** for a struct.
///
/// This macro creates a chainable API for setting each field.
/// Once all fields are set, `.build()` validates and returns `Result<Self, &str>`.
///
/// ### Supported field attributes:
///
/// - `#[set(value = "...")]`  
///   Sets a default value for the field when calling `Struct::new()`.  
///   If omitted, the field will use the default from `Default::default()`.
///
/// - `#[range(min = x, max = y, err = "...")]`  
///   Adds compile-time generated range validation (numeric types only).
///
/// - `#[build(pattern = "...", err = "...")]`  
///   Applies regex validation to the field on `.build()`.
///
/// - `#[skip_trait]`  
///   Prevents automatic implementation of helper traits.
///
/// ### Example:
///
/// ```rust
/// use kenzu::Builder;
///
/// #[derive(Builder)]
/// pub struct User {
///     pub id: String,
///     #[set(value = "default_name")]
///     pub name: String,
///     #[build(pattern = r"^.+@.+\..+$", err = "invalid email")]
///     pub email: String,
///     #[range(min = 18, max = 99, err = "invalid age")]
///     pub age: u8,
/// }
///
/// let user = User::new()
///     .id("001")
///     .name("Alice")
///     .email("alice@example.com")
///     .age(30)
///     .build()
///     .unwrap();
/// ```
#[proc_macro_derive(Builder, attributes(range, set, build, skip_trait))]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    //add_traits_to_generics(&mut input);
    let generate_build = generate_build(&input);
    generate_build.into()
}

/// Derive macro `M_Builder` generates a **mutable builder** for a struct,
/// allowing progressive field updates and direct value access.
///
/// Unlike `Builder`, this version uses mutable method calls (`&mut self`)
/// and doesn't consume `self` when calling `.build()`.
///
/// Useful for structs with lifetimes or scenarios where you want to inspect
/// and mutate the builder mid-construction.
///
/// ### Supported field attributes:
///
/// - `#[set(value = "...")]`  
///   Sets a default value at construction time (`Struct::new()`).
///
/// - `#[set(skip)]`  
///   Skips generating the builder method for this field.
///   Useful for computed or internal fields.
///
/// - `#[range(min = x, max = y, err = "...")]`  
///   Adds numeric range validation.
///
/// - `#[build(pattern = "...", err = "...")]`  
///   Applies regex validation to string fields.
///
/// - `#[skip_trait]`  
///   Prevents automatic implementation of helper traits.
///
/// ### Example:
///
/// ```rust
/// use kenzu::M_Builder;
///
/// #[derive(M_Builder)]
/// pub struct User<'a> {
///     #[set(value = "uuid")]
///     pub id: String,
///     #[set(value = "default")]
///     pub name: String,
///     pub password: String,
///     #[build(pattern = r"^.+@.+\..+$", err = "invalid email")]
///     #[set(value = "email@example.com")]
///     pub email: String,
///     pub fix: &'a str,
///     #[set(value = 18)]
///     pub age: u8,
///     #[set(skip)]
///     pub code: u8,
///     pub def: String,
/// }
///
/// let mut user = User::new();
/// user.id("001")
///     .name("Bob")
///     .password("hunter2")
///     .email("bob@example.com")
///     .fix("static_ref")
///     .age(42)
///     .build()
///     .unwrap();
/// ```
#[proc_macro_derive(M_Builder, attributes(range, set, build, skip_trait))]
pub fn mutating_builder(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as DeriveInput);
    add_traits_to_generics(&mut input);
    let generate_build = generate_mutating_build(&input);
    generate_build.into()
}
