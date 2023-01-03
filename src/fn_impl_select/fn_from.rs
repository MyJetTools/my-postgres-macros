use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;
use types_reader::StructProperty;

pub fn fn_from(fields: &[StructProperty]) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.len());

    for field in fields {
        let name_ident = field.get_field_name_ident();

        let type_ident = field.ty.get_token_stream();

        let db_field_name = field.get_db_field_name()?;
        let db_field_name = db_field_name.as_str();

        let metadata = crate::render_field_value::render_metadata(field);

        result.push(quote! {
            #name_ident: #type_ident::from_db_row(db_row, #db_field_name, &#metadata),
        });
    }

    Ok(result)
}
