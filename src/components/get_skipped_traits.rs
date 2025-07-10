use std::collections::HashSet;
use syn::Attribute;

#[derive(Debug, Default)]
pub struct SkippedTraits {
    pub skipped: HashSet<String>,
}

impl SkippedTraits {
    pub fn is_skipped(&self, trait_name: &str) -> bool {
        self.skipped.contains(trait_name)
    }
}

pub fn get_skipped_traits(attributes: &[Attribute]) -> SkippedTraits {
    let mut skipped = HashSet::new();

    for attr in attributes {
        if attr.path().is_ident("skip_trait") {
            let _ = attr.parse_nested_meta(|meta| {
                if let Some(ident) = meta.path.get_ident() {
                    if meta.input.is_empty() {
                        skipped.insert(ident.to_string());
                        Ok(())
                    } else {
                        Err(meta.error("Trait skip does not accept a value"))
                    }
                } else {
                    Err(meta.error("Expected an identifier for trait"))
                }
            });
        }
    }

    SkippedTraits { skipped }
}
