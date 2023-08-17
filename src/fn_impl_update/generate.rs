use proc_macro::TokenStream;
use types_reader::{StructProperty, TypeName};

use crate::postgres_struct_ext::PostgresStructPropertyExt;
use quote::quote;

use super::update_fields::UpdateFields;

pub fn generate(ast: &syn::DeriveInput) -> Result<TokenStream, syn::Error> {
    let type_name = TypeName::new(ast);

    let struct_name = type_name.get_type_name();

    let fields = StructProperty::read(ast)?;

    let fields = crate::postgres_struct_ext::filter_fields(fields)?;

    let fields = UpdateFields::new(fields);

    let get_field_value_case = super::fn_get_field_value::fn_get_field_value(&fields)?;

    let fields_amount = fields.get_update_fields_amount();

    let where_impl = crate::fn_impl_where_model::generate_implementation(
        &type_name,
        fields.get_fields_with_primary_key(),
        None,
        None,
    )?;

    let fn_get_column_name = get_columns(&fields)?;

    Ok(quote! {

        impl my_postgres::sql_update::SqlUpdateModel for #struct_name{
            fn get_fields_amount() -> usize{
                #fields_amount
            }

            fn get_column_name(no: usize) -> my_postgres::ColumnName{
                #fn_get_column_name
            }

            fn get_field_value(&self, no: usize) -> my_postgres::sql_update::SqlUpdateModelValue{
                match no{
                    #(#get_field_value_case)*
                    _=>panic!("no such field with number {}", no)
                }

            }

        }

        #where_impl
    }
    .into())
}

fn get_columns(fields: &UpdateFields) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut line = Vec::with_capacity(fields.get_fields_amount());
    let mut no: usize = 0;
    for field in fields.get_fields_with_no_primary_key() {
        let db_field_name = field.get_db_column_name_as_string()?;

        line.push(quote::quote!(#no=>#db_field_name.into(),));
        if field.is_primary_key() {
            continue;
        }

        no += 1;
    }

    let result = quote! {
        match no{
          #(#line)*
          _=>panic!("no such field with number {}", no)
        }
    };

    Ok(result)
}
