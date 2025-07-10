use syn::{Attribute, LitStr};

#[derive(Debug, Default)]
pub struct Builder {
    pub pattern: Option<String>,
    pub err: Option<String>,
}

pub fn get_build(attributes: &Vec<Attribute>) -> Builder {
    let mut builder = Builder::default();

    for attr in attributes {
        if attr.path().is_ident("build") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("pattern") {
                    if let Ok(parsed_value) = meta.value()?.parse::<LitStr>() {
                        builder.pattern = Some(parsed_value.value());
                    }
                    Ok(())
                } else if meta.path.is_ident("err") {
                    if let Ok(parsed_value) = meta.value()?.parse::<LitStr>() {
                        builder.err = Some(parsed_value.value());
                    }
                    Ok(())
                } else {
                    Err(meta.error("Atributo não suportado"))
                }
            })
            .unwrap();
        }
    }

    builder
}
