use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::postgres_utils::filter_fields(StructProperty::read(ast));

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl<'s> my_postgres::sql_insert::SqlInsertModel<'s> for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn get_fields_amount()->usize{");
    result.push_str(fields.len().to_string().as_str());
    result.push_str("}\n");

    result.push_str("fn get_field_name(no: usize) -> &'static str{");
    super::fn_get_field_name::fn_get_field_name(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("fn get_field_value(no: usize) -> &'static str{");
    super::fn_get_field_value::fn_get_field_value(&mut result, &fields);
    result.push_str("}\n");

    // implementation of trait
    result.push_str("}\n");
    result.parse().unwrap()
}
