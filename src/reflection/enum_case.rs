use std::collections::HashMap;

use super::MyAttribute;

pub struct EnumCase {
    pub attrs: HashMap<String, MyAttribute>,
    pub name: String,
}

impl EnumCase {
    pub fn read(ast: &syn::DeriveInput) -> Vec<Self> {
        let mut result = Vec::new();

        if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
            for varian in variants {
                result.push(EnumCase {
                    name: varian.ident.to_string(),
                    attrs: MyAttribute::new(&varian.attrs),
                });
            }
        } else {
            panic!("Enum Only")
        };

        result
    }
}
