use quote::quote;
use types_reader::StructProperty;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn get_group_by_fields(
    fields: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut group_by = Vec::with_capacity(fields.len());

    for prop in fields {
        if prop.attrs.has_attr("group_by") {
            group_by.push(prop.get_db_field_name_as_string()?);
            continue;
        }
    }

    if group_by.is_empty() {
        return Ok(quote! { None }.into());
    }

    let mut group_by_str = String::new();

    group_by_str.push_str(" GROUP BY");
    for prop in group_by {
        group_by_str.push(' ');
        group_by_str.push_str(prop.as_str());
    }

    Ok(quote! { Some(#group_by_str) }.into())
}
