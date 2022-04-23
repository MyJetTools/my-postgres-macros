use proc_macro::TokenStream;

pub fn generate_read_single_row(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");
    result.push_str("pub fn read_single_row(rows: &[tokio_postgres::Row])->Option<Self>{");
    super::generate_single_record_reading::generate_single_record_reading(&mut result, &fields);
    result.push_str("}\n");
    result.push_str("}\n");

    result.parse().unwrap()
}
