use std::collections::BTreeMap;

use proc_macro2::{Ident, TokenStream};

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

    let mut indexes_list = BTreeMap::new();

    for field in fields {
        let field_name = field.get_db_field_name_as_string();
        let sql_type = get_sql_type(field, &field.ty)?;
        let is_option = field.ty.is_option();
        if let Some(value) = field.get_primary_key_id()? {
            if primary_keys.contains_key(&value) {
                return Err(syn::Error::new_spanned(
                    field.field,
                    format!("Primary key id {} is already used", value),
                ));
            }
            primary_keys.insert(value, field_name.clone());
        };

        if let Some(indexes) = field.get_index_attrs()? {
            for index in indexes {
                if !indexes_list.contains_key(&index.index_name) {
                    indexes_list.insert(index.index_name.clone(), BTreeMap::new());
                }

                let index_by_name = indexes_list.get_mut(&index.index_name).unwrap();

                if index_by_name.contains_key(&index.id) {
                    panic!("Duplicate index id {} for index {}", index.id, index.name);
                }

                index_by_name.insert(index.id, index);
            }
        }

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

    let indexes = if indexes_list.is_empty() {
        quote::quote!(None)
    } else {
        let mut quotes: Vec<TokenStream> = Vec::new();

        for (index_name, index_data) in indexes_list {
            let mut fields = Vec::new();

            let mut is_unique = false;

            for index_data in index_data.values() {
                is_unique = index_data.is_unique;
                let name = &index_data.name;

                let order = match index_data.order.as_str() {
                    "ASC" => quote::quote!(IndexOrder::Asc),
                    "DESC" => quote::quote!(IndexOrder::Desc),
                    _ => panic!("Unknown index order {}", index_data.order),
                };

                fields.push(quote::quote!(IndexField { name: #name.into(), order: #order }));
            }

            quotes.push(quote::quote!(result.insert(#index_name.to_string(), IndexSchema::new(#is_unique, vec![#(#fields,)*]));));
        }

        quote::quote! {
            let mut result = std::collections::HashMap::new();
            #(#quotes;)*

            Some(result)
        }
    };

    let result = quote::quote! {

        impl my_postgres::table_schema::TableSchemaProvider for #struct_name{
            const PRIMARY_KEY_COLUMNS: Option<&'static [&'static str]> = #primary_keys;
            fn get_columns() -> Vec<my_postgres::table_schema::TableColumn>{
                use my_postgres::table_schema::*;
                vec![#(#result),*]
            }
            fn get_indexes() -> Option<std::collections::HashMap<String, my_postgres::table_schema::IndexSchema>>{
                use my_postgres::table_schema::*;
                #indexes
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
    if let PropertyType::Option(ty) = ty {
        return get_sql_type(field, ty);
    }

    let ty_token = ty.get_token_stream_with_generics();

    let meta_data = field.get_field_metadata();

    Ok(quote::quote! {#ty_token:: get_sql_type(#meta_data)})

    /*
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
     */
}
