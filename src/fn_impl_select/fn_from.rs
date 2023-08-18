use crate::postgres_struct_ext::PostgresStructPropertyExt;
use quote::quote;
use types_reader::{PropertyType, StructProperty};

pub fn fn_from(fields: &[StructProperty]) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.len());

    for field in fields {
        let name_ident = field.get_field_name_ident();

        let db_column_name = field.get_db_column_name_as_string()?;

        let metadata = field.get_field_metadata()?;

        let reading = if let PropertyType::OptionOf(sub_prop) = &field.ty {
            let type_ident = sub_prop.get_token_stream();
            quote!(#type_ident::from_db_row_opt(db_row, #db_column_name, &#metadata))
        } else {
            let type_ident = field.ty.get_token_stream();
            quote!(#type_ident::from_db_row(db_row, #db_column_name, &#metadata))
        };

        result.push(quote! {
            #name_ident: #reading,
        });
    }

    Ok(result)
}
