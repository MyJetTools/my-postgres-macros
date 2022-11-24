use proc_macro::TokenStream;
use types_reader::{PropertyType, StructProperty};

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let fields = crate::postgres_utils::filter_fields(StructProperty::read(ast));

    let mut has_str = false;

    for field in &fields {
        if let PropertyType::Str = field.ty {
            has_str = true;
            break;
        }
    }

    let struct_name = name.to_string();
    let mut result = String::new();
    generate_implementation(
        &mut result,
        struct_name.as_str(),
        fields.as_slice(),
        has_str,
    );

    result.parse().unwrap()
}

pub fn generate_implementation(
    result: &mut String,
    struct_name: &str,
    fields: &[StructProperty],
    has_str: bool,
) {
    result.push_str("impl<'s> my_postgres::sql_where::SqlWhereModel<'s> for ");
    result.push_str(struct_name);

    if has_str {
        result.push_str("<'s>");
    }
    result.push_str(" {\n");

    result.push_str("fn get_max_fields_amount() -> usize {");
    result.push_str(fields.len().to_string().as_str());
    result.push_str("}\n");

    result.push_str("fn get_field_value(&'s self, no: usize) -> my_postgres::SqlWhereValue<'s> {");
    super::fn_get_field_value::fn_get_field_value(result, &fields);
    result.push_str("}\n");

    result.push_str("}\n");
}
