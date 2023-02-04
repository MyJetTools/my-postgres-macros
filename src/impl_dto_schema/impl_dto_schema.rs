use proc_macro2::Ident;

use types_reader::{PropertyType, StructProperty};

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn impl_dto_schema(
    struct_name: &Ident,
    fields: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = Vec::new();

    for field in fields {
        let field_name = field.get_field_name_ident();
        let sql_type = get_sql_type(field, &field.ty)?;
        let is_option = field.ty.is_option();
        result.push(quote::quote! {
            DtoColumn{
                name: #field_name,
                sql_type: #sql_type,
                is_primary_key: false,
                is_nullable: #is_option

            }
        });
    }

    let result = quote::quote! {
        impl my_postgres::sql_schema::DtoSchema for #struct_name{
            fn get_columns() -> Vec<DtoColumn>{
                use my_postgres::sql_schema::*;
                vec![#(#result),*]
            }
        }
    }
    .into();

    Ok(result)
}

fn get_sql_type(
    field: &StructProperty,
    ty: &PropertyType,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let result = match &ty {
        types_reader::PropertyType::U8 => quote::quote!(SqlType::SmallInt),
        types_reader::PropertyType::I8 => quote::quote!(SqlType::SmallInt),
        types_reader::PropertyType::U16 => quote::quote!(SqlType::Integer),
        types_reader::PropertyType::I16 => quote::quote!(SqlType::SmallInt),
        types_reader::PropertyType::U32 => quote::quote!(SqlType::Integer),
        types_reader::PropertyType::I32 => quote::quote!(SqlType::Integer),
        types_reader::PropertyType::U64 => quote::quote!(SqlType::BigInt),
        types_reader::PropertyType::I64 => quote::quote!(SqlType::BigInt),
        types_reader::PropertyType::F32 => quote::quote!(SqlType::Real),
        types_reader::PropertyType::F64 => quote::quote!(SqlType::Double),
        types_reader::PropertyType::USize => quote::quote!(SqlType::BigInt),
        types_reader::PropertyType::ISize => quote::quote!(SqlType::BigInt),
        types_reader::PropertyType::String => quote::quote!(SqlType::Text),
        types_reader::PropertyType::Str => quote::quote!(SqlType::Text),
        types_reader::PropertyType::Bool => quote::quote!(SqlType::Bool),
        types_reader::PropertyType::DateTime => {
            let sql_type = field.get_sql_type()?;

            let sql_type = sql_type.as_str();

            if sql_type == "timestamp" {
                quote::quote!(SqlType::Timestamp)
            } else if sql_type == "bigint" {
                quote::quote!(SqlType::BigInt)
            } else {
                return Err(syn::Error::new_spanned(
                    field.field,
                    format!("Sql type must be 'timestamp' or 'bigint'"),
                ));
            }
        }

        types_reader::PropertyType::OptionOf(sub_type) => get_sql_type(field, &sub_type)?,
        types_reader::PropertyType::VecOf(_) => quote::quote!(SqlType::Json),
        types_reader::PropertyType::Struct(_, _) => quote::quote!(SqlType::Json),
        types_reader::PropertyType::HashMap(_, _) => quote::quote!(SqlType::Json),
    };

    Ok(result)
}
