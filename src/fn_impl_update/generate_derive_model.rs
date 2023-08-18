use proc_macro2::TokenStream;

use crate::{postgres_struct_ext::PostgresStructPropertyExt, struct_name::StructName};

use super::update_fields::UpdateFields;

pub fn generate_derive_model(
    struct_name: &TokenStream,
    type_name: StructName,
    update_fields: UpdateFields,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let fields_amount = update_fields.get_update_fields().len();

    let fn_get_column_name = get_columns(&update_fields)?;

    let get_field_value_case = super::fn_get_field_value::fn_get_field_value(&update_fields)?;

    let where_impl = crate::fn_impl_where_model::generate_implementation(
        type_name,
        update_fields.get_where_fields().iter().map(|x| *x),
        None,
        None,
    )?;

    Ok(quote::quote! {

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
    for field in fields.get_update_fields() {
        let db_column_name = field.get_db_column_name_as_string()?;

        line.push(quote::quote!(#no=>#db_column_name.into(),));
        no += 1;
    }

    let result = quote::quote! {
        match no{
          #(#line)*
          _=>panic!("no such field with number {}", no)
        }
    };

    Ok(result)
}
