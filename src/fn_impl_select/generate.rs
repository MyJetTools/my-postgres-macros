use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl<'s> my_postgres::SelectEntity<'s> for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn from_db_row(row: &tokio_postgres::Row) -> Self {");
    super::fn_from_db_row::fn_from_db_row(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("fn get_select_fields() -> &'static str {");
    super::fn_select_fields(&mut result, &fields);
    result.push_str("}\n");

    result.push_str(
        "fn get_order_by_fields() -> Option<my_postgres::sql_select::OrderByFields<'s>> {",
    );
    super::fn_get_order_by_fields::fn_get_order_by_fields(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
