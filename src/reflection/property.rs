use std::collections::HashMap;

use super::{MyAttribute, PropertyType};

pub struct StructProperty {
    pub name: String,
    pub ty: PropertyType,
    pub attrs: HashMap<String, MyAttribute>,
}

impl StructProperty {
    pub fn read(ast: &syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        let fields = if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(ref fields),
            ..
        }) = ast.data
        {
            fields
        } else {
            panic!("Struct Only")
        };

        for field in &fields.named {
            let attrs = MyAttribute::new(&field.attrs);

            result.push(Self {
                name: field.ident.as_ref().unwrap().to_string(),
                ty: PropertyType::new(field),
                attrs,
            })
        }

        result
    }

    pub fn get_db_field_name(&self) -> &str {
        if let Some(attr) = self.attrs.get("db_field_name") {
            match attr.get_value("name") {
                Some(result) => return result,
                None => panic!("Attribute db_field_name must have a name"),
            }
        }

        self.name.as_str()
    }

    pub fn is_key(&self) -> bool {
        self.attrs.get("key").is_some()
    }
}
