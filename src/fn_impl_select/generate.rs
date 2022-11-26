use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::postgres_utils::filter_fields(StructProperty::read(ast));

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_postgres::sql_select::SelectEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn fill_select_fields(sql: &mut String) {");
    super::fn_fill_select_fields::fn_fill_select_fields(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("fn fill_order_by_fields(sql: &mut String) {");
    super::fn_fill_order_by::fn_fill_order_by(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("fn fill_group_by_fields(sql: &mut String){");
    super::fn_fill_group_by::fn_fill_group_by(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("fn from(db_row: &tokio_postgres::Row) -> Self {");
    super::fn_from::fn_from(&mut result, &fields);
    result.push_str("}");

    result.push_str("}\n");

    result.parse().unwrap()
}
