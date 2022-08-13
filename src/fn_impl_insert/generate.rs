use proc_macro::TokenStream;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::reflection::StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl my_postgres::InsertEntity for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str(
        "fn populate<'s>(&'s self, sql_builder: &mut dyn my_postgres::code_gens::insert::InsertCodeGen<'s>){",
    );
    super::fn_insert::fn_insert(&mut result, &fields);
    result.push_str("}\n");
    result.push_str("}\n");
    result.parse().unwrap()
}
