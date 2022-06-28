use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("pub async fn bulk_insert_or_update(entities: Vec<Self>, client: &mut tokio_postgres::Client, table_name: &str, pk_name: &str,) -> Result<(), tokio_postgres::Error> {");
    super::fn_bulk_insert_or_update::fn_bulk_insert_or_update(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
