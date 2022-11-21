use proc_macro::TokenStream;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = StructProperty::read(ast);

    let struct_name = name.to_string();

    let mut result = String::new();

    result.push_str("impl<'s> my_postgres::SqlWhereData<'s> for ");
    result.push_str(struct_name.as_str());
    result.push_str(" {\n");

    result.push_str("fn get_max_fields_amount() -> usize {");
    result.push_str(fields.len().to_string().as_str());
    result.push_str("}\n");

    result.push_str("fn get_field_value(&'s self, no: usize) -> my_postgres::InputDataValue<'s> {");
    super::fn_get_field_value::fn_get_field_value(&mut result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");

    result.parse().unwrap()
}
