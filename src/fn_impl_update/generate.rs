use proc_macro::TokenStream;
use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut has_str = false;

    for field in &fields {
        if let PropertyType::Str = field.ty {
            has_str = true;
            break;
        }
    }

    let mut primary_key_amount = 0;

    for field in &fields {
        if field.is_primary_key() {
            primary_key_amount += 1;
        }
    }

    let mut result = String::new();

    result.push_str("impl my_postgres::sql_update::SqlUpdateModel for ");
    result.push_str(struct_name.as_str());
    if has_str {
        result.push_str("<'s>");
    }
    result.push_str(" {\n");

    result.push_str("fn get_fields_amount() -> usize{");
    result.push_str((fields.len() - primary_key_amount).to_string().as_str());
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
