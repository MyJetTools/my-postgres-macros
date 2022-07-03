use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_postgres::DeleteEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("async fn bulk_delete(&self, sql_builder: &mut dyn crate::code_gens::delete::DeleteCodeGen){");
    super::fn_delete::fn_delete(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
