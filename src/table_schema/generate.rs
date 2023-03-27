use std::collections::BTreeMap;

use proc_macro2::Ident;

use types_reader::{PropertyType, StructProperty};

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &ast.ident;

    let fields = match StructProperty::read(ast) {
        Ok(fields) => fields,
        Err(e) => return e.into_compile_error().into(),
    };

    let fields = super::utils::filter_table_schema_fields(&fields);
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
    fields: &Vec<&StructProperty>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut result = Vec::new();

    let mut primary_keys = BTreeMap::new();

    for field in fields {
        let field_name = field.get_db_field_name_as_string();
        let sql_type = get_sql_type(field, &field.ty)?;
        let is_option = field.ty.is_option();
        if let Some(value) = field.get_primary_key_id() {
            if primary_keys.contains_key(&value) {
                return Err(syn::Error::new_spanned(
                    field.field,
                    format!("Primary key id {} is already used", value),
                ));
            }
            primary_keys.insert(value, field_name.clone());
        };

        result.push(quote::quote! {
            TableColumn{
                name: #field_name.to_string(),
                sql_type: #sql_type,
                is_nullable: #is_option,
                default: None
            }
        });
    }

    let primary_keys = if primary_keys.is_empty() {
        quote::quote!(None)
    } else {
        let mut result = Vec::new();
        for (_, value) in primary_keys {
            result.push(value);
        }
        quote::quote!(Some(&[#(#result),*]))
    };

    let result = quote::quote! {

        impl my_postgres::table_schema::TableSchemaProvider for #struct_name{
            const PRIMARY_KEY_COLUMNS: Option<&'static [&'static str]> = #primary_keys;
            fn get_columns() -> Vec<my_postgres::table_schema::TableColumn>{
                use my_postgres::table_schema::*;
                vec![#(#result),*]
            }
            fn get_indexes() -> Option<std::collections::HashMap<String, my_postgres::table_schema::IndexSchema>>{
                None
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
        types_reader::PropertyType::U8 => quote::quote!(TableColumnType::SmallInt),
        types_reader::PropertyType::I8 => quote::quote!(TableColumnType::SmallInt),
        types_reader::PropertyType::U16 => quote::quote!(TableColumnType::Integer),
        types_reader::PropertyType::I16 => quote::quote!(TableColumnType::SmallInt),
        types_reader::PropertyType::U32 => quote::quote!(TableColumnType::Integer),
        types_reader::PropertyType::I32 => quote::quote!(TableColumnType::Integer),
        types_reader::PropertyType::U64 => quote::quote!(TableColumnType::BigInt),
        types_reader::PropertyType::I64 => quote::quote!(TableColumnType::BigInt),
        types_reader::PropertyType::F32 => quote::quote!(TableColumnType::Real),
        types_reader::PropertyType::F64 => quote::quote!(TableColumnType::Double),
        types_reader::PropertyType::USize => quote::quote!(TableColumnType::BigInt),
        types_reader::PropertyType::ISize => quote::quote!(TableColumnType::BigInt),
        types_reader::PropertyType::String => quote::quote!(TableColumnType::Text),
        types_reader::PropertyType::Str => quote::quote!(TableColumnType::Text),
        types_reader::PropertyType::Bool => quote::quote!(TableColumnType::Boolean),
        types_reader::PropertyType::DateTime => {
            let sql_type = field.get_sql_type()?;

            let sql_type = sql_type.as_str();

            if sql_type == "timestamp" {
                quote::quote!(TableColumnType::Timestamp)
            } else if sql_type == "bigint" {
                quote::quote!(TableColumnType::BigInt)
            } else {
                return Err(syn::Error::new_spanned(
                    field.field,
                    format!("Sql type must be 'timestamp' or 'bigint'"),
                ));
            }
        }

        types_reader::PropertyType::OptionOf(sub_type) => get_sql_type(field, &sub_type)?,
        types_reader::PropertyType::VecOf(_) => quote::quote!(TableColumnType::Json),
        types_reader::PropertyType::Struct(_, _) => quote::quote!(TableColumnType::Json),
        types_reader::PropertyType::HashMap(_, _) => quote::quote!(TableColumnType::Json),
    };

    Ok(result)
}
