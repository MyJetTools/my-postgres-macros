use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_postgres::InsertOrUpdateEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn populate<'s>(&'s self, sql_builder: &mut my_postgres::code_gens::insert_or_update::InsertOrUpdateBuilder<'s>) {");
    super::fn_insert_or_update::fn_insert_or_update(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
