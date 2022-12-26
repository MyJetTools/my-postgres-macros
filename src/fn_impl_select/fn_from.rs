use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;
use types_reader::StructProperty;

pub fn fn_from(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::with_capacity(fields.len());

    for field in fields {
        let name_ident = field.get_field_name_ident();

        let type_ident = field.ty.get_token_stream();

        let db_field_name = field.get_db_field_name();

        let sql_type = super::fill_sql_type(field);

        result.push(quote! {
            #name_ident: #type_ident::from_db_row(db_row, #db_field_name, #sql_type),
        });
    }

    result
}
