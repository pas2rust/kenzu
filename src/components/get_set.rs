use proc_macro2::TokenStream;
use syn::Attribute;

#[derive(Debug)]
pub struct SetValue {
    pub value: Option<TokenStream>,
    pub skip: bool,
}

pub fn get_set_value(attributes: &[Attribute]) -> SetValue {
    let mut value = None;
    let mut skip = false;

    for attr in attributes {
        if attr.path().is_ident("set") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("value") {
                    // Parse #[set(value = ...)]
                    let parsed = meta.value()?.parse::<TokenStream>()?;
                    value = Some(parsed);
                    Ok(())
                } else if meta.path.is_ident("skip") {
                    // Parse #[set(skip)]
                    // If it's path-only like #[set(skip)], input.is_empty() == true
                    if meta.input.is_empty() {
                        skip = true;
                        Ok(())
                    } else {
                        Err(meta.error("Attribute 'skip' does not accept a value"))
                    }
                } else {
                    Err(meta.error("Unsupported attribute inside #[set(...)]"))
                }
            });
        }
    }

    SetValue { value, skip }
}
