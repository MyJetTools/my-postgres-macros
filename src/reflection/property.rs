use std::collections::HashMap;

use super::{MyAttribute, PropertyType};

pub const ATTR_PRIMARY_KEY: &str = "primary_key";
pub const ATTR_DB_FIELD_NAME: &str = "db_field_name";
//pub const ATTR_IGNORE_IF_NULL: &str = "ignore_if_null";

pub const ATTR_TIMESTAMP: &str = "timestamp";
pub const ATTR_BIGINT: &str = "bigint";

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
        if let Some(attr) = self.attrs.get(ATTR_DB_FIELD_NAME) {
            match attr.get_value("name") {
                Some(result) => return result,
                None => panic!("Attribute db_field_name must have a name"),
            }
        }

        self.name.as_str()
    }

    pub fn is_primary_key(&self) -> bool {
        self.attrs.get(ATTR_PRIMARY_KEY).is_some()
    }

    pub fn has_timestamp_attr(&self) -> bool {
        self.attrs.get(ATTR_TIMESTAMP).is_some()
    }

    pub fn has_bigint_attr(&self) -> bool {
        self.attrs.get(ATTR_BIGINT).is_some()
    }

    /*
    pub fn has_ignore_if_null_attr(&self) -> bool {
        self.attrs.get(ATTR_IGNORE_IF_NULL).is_some()
    }
     */
}
