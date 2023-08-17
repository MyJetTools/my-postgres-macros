use proc_macro::TokenStream;
use quote::quote;
use types_reader::StructProperty;

use crate::{e_tag::GetETag, postgres_struct_ext::PostgresStructPropertyExt};

use super::insert_fields::InsertFields;

pub fn generate(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;

    let fields = StructProperty::read(ast)?;

    let fields = crate::postgres_struct_ext::filter_fields(fields)?;

    let fields = InsertFields::new(fields);

    let fields_amount = fields.get_fields_amount();

    let fn_get_column_name = fn_get_column_name(fields.as_slice())?;

    let get_field_value = fn_get_field_value(fields.as_slice())?;

    let e_tag = fields.get_e_tag();

    let e_tag_methods = crate::e_tag::generate_e_tag_methods(e_tag);

    let result = quote! {
        impl my_postgres::sql_insert::SqlInsertModel for #name{

            fn get_fields_amount()->usize{
                #fields_amount
            }

            fn get_column_name(no: usize) -> my_postgres::ColumnName{
                match no{
                    #(#fn_get_column_name)*
                    _=>panic!("no such field with number {}", no)
                }
            }

            fn get_field_value(&self, no: usize) -> my_postgres::sql_update::SqlUpdateModelValue{
                match no{
                    #(#get_field_value)*
                    _=>panic!("no such field with number {}", no)
                }
            }

            #e_tag_methods

        }

    }
    .into();

    Ok(result)
}

pub fn fn_get_column_name(
    fields: &[StructProperty],
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::new();
    for (i, prop) in fields.iter().enumerate() {
        let field_name = prop.get_db_column_name_as_string()?;

        result.push(quote! (#i=>#field_name.into(),).into());
    }
    Ok(result)
}

pub fn fn_get_field_value(
    fields: &[StructProperty],
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let value = field.render_field_value(true)?;

        result.push(quote! (#i => #value,));
    }
    Ok(result)
}
