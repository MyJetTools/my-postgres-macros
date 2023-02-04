use proc_macro2::Ident;

use types_reader::{PropertyType, StructProperty};

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &ast.ident;

    let fields = match StructProperty::read(ast) {
        Ok(fields) => fields,
        Err(e) => return e.into_compile_error().into(),
    };

    let fields = match crate::postgres_struct_ext::filter_fields(fields) {
        Ok(result) => result,
        Err(err) => {
            return err.into();
        }
    };

    let db_columns = match impl_db_columns(struct_name, &fields) {
        Ok(db_columns) => db_columns,
        Err(err) => {
            return err.into_compile_error().into();
        }
    };

    quote::quote!(#db_columns).into()
}

fn impl_db_columns(
    struct_name: &Ident,
    fields: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = Vec::new();

    for field in fields {
        let field_name = field.get_db_field_name_as_string();
        let sql_type = get_sql_type(field, &field.ty)?;
        let is_option = field.ty.is_option();
        let is_primary_key = field.is_primary_key();
        result.push(quote::quote! {
            DbColumn{
                name: #field_name,
                sql_type: #sql_type,
                is_primary_key: #is_primary_key,
                is_nullable: #is_option

            }
        });
    }

    let result = quote::quote! {
        impl my_postgres::db_schema::DbSchema for #struct_name{
            fn get_columns() -> Vec<my_postgres::db_schema::DbColumn>{
                use my_postgres::db_schema::*;
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
        types_reader::PropertyType::Bool => quote::quote!(SqlType::Boolean),
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
