use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_postgres::BulkSelectInputData for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn where_line() -> &'static str {");
    super::fn_where_line::fn_where_line(&mut result, &fields);
    result.push_str("}\n");

    result.push_str(
        "fn get_param_value(&self, no: usize) -> &(dyn tokio_postgres::types::ToSql + Sync) {",
    );
    super::fn_get_param_value::fn_get_param_value(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
