use proc_macro::TokenStream;
use types_reader::{PropertyType, StructProperty};

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let src_fields = StructProperty::read(ast);

    let mut limit = None;
    let mut offset = None;

    let mut fields = Vec::with_capacity(src_fields.len());

    for field in src_fields {
        if field.attrs.has_attr("limit") {
            limit = Some(field);
        } else if field.attrs.has_attr("offset") {
            offset = Some(field);
        } else {
            fields.push(field);
        }
    }

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
        limit,
        offset,
    );

    result.parse().unwrap()
}

pub fn generate_implementation(
    result: &mut String,
    struct_name: &str,
    fields: &[StructProperty],
    has_str: bool,
    limit: Option<StructProperty>,
    offset: Option<StructProperty>,
) {
    result.push_str("impl<'s> my_postgres::sql_where::SqlWhereModel<'s> for ");
    result.push_str(struct_name);

    if has_str {
        result.push_str("<'s>");
    }
    result.push_str(" {\n");

    result.push_str("fn fill_where(&self, sql: &mut String, params: &mut Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,) {");
    super::fn_fill_where::fn_fill_where(result, &fields);
    result.push_str("}\n");

    result.push_str("fn get_limit(&self) -> Option<usize> {");

    if let Some(limit) = limit {
        result.push_str("self.");
        result.push_str(limit.name.as_str());
    } else {
        result.push_str("None");
    }

    result.push_str("}\n");

    result.push_str("fn get_offset(&self) -> Option<usize> {");
    if let Some(offset) = offset {
        result.push_str("self.");
        result.push_str(offset.name.as_str());
    } else {
        result.push_str("None");
    }
    result.push_str("}\n");

    result.push_str("}\n");
}
