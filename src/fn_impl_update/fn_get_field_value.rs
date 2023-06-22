use crate::postgres_struct_ext::PostgresStructPropertyExt;
use proc_macro2::TokenStream;
use quote::quote;

use super::update_fields::UpdateFields;

pub fn fn_get_field_value(fields: &UpdateFields) -> Result<Vec<TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.get_fields_amount());

    let mut i: usize = 0;
    for field in fields.get_fields_with_no_primary_key() {
        let sql_update_value = field.render_field_value(true)?;
        result.push(
            quote! {
                #i => #sql_update_value,
            }
            .into(),
        );
        i += 1;
    }

    Ok(result)
}
