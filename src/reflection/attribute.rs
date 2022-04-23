use std::collections::HashMap;

use super::attr_params_parser::AttrParamsParser;

#[derive(Clone)]
pub struct MyAttribute {
    pub name: String,
    pub content: Option<Vec<u8>>,
}

impl MyAttribute {
    pub fn new(src: &[syn::Attribute]) -> HashMap<String, MyAttribute> {
        let mut result = HashMap::new();
        for attr in src {
            for segment in attr.path.segments.iter() {
                let attr_id = segment.ident.to_string();
                let attr_data = attr.tokens.to_string();
                let attr_data = if attr_data == "" {
                    None
                } else {
                    Some(attr_data.into_bytes())
                };

                result.insert(
                    attr_id.to_string(),
                    MyAttribute {
                        name: attr_id,
                        content: attr_data,
                    },
                );
            }
        }

        result
    }

    pub fn get_value(&self, key_to_find: &str) -> Option<&str> {
        let content = self.content.as_ref()?;
        for (key, value) in AttrParamsParser::new(content.as_slice()) {
            if key_to_find == key {
                return Some(value);
            }
        }

        None
    }
}
