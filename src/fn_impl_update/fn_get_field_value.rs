use crate::postgres_utils::PostgresStructPropertyExt;
use proc_macro2::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn fn_get_field_value(fields: &[StructProperty]) -> Vec<TokenStream> {
    let mut result = Vec::with_capacity(fields.len());

    let mut i: usize = 0;
    for field in fields {
        if field.is_primary_key() {
            continue;
        }

        let db_field_name = field.get_db_field_name();

        let value = crate::get_field_value::get_field_value(field);

        result.push(
            quote! {
                #i => my_postgres::sql_update::SqlUpdateValue{ name: #db_field_name, value: #value}
            }
            .into(),
        );
        i += 1;
    }

    result
}
