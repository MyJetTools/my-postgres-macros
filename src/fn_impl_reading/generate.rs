use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("pub fn from_db_row(row: &tokio_postgres::Row) -> Self {");
    super::fn_from_db_row::fn_from_db_row(&mut result, &fields);
    result.push_str("}\n");

    result.push_str(super::functions::QUERY_SINGLE_ROW);
    result.push_str(super::functions::QUERY_ROWS);

    result.push_str("}\n");

    result.parse().unwrap()
}
