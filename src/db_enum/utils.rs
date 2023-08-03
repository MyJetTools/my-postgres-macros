use std::str::FromStr;

use proc_macro2::{Ident, TokenStream};
use types_reader::EnumCase;

use crate::postgres_enum_ext::PostgresEnumExt;

pub fn render_reading_db_row_metadata_model() -> proc_macro2::TokenStream {
    quote::quote! {

        let model_field_name = if let Some(metadata) = metadata{
            if metadata.related_column_name.is_none(){
             panic!("Metadata model field_name is none");
            }
            metadata.related_column_name.unwrap()
         }
         else{
             panic!("Metadata is not defined for enum with model");
         };

         let model:String = row.get(model_field_name);
    }
}

pub fn render_update_value_provider_fn_body() -> proc_macro2::TokenStream {
    quote::quote! {
        let (name, model) = self.to_str();
        let index_name = params.push_static_str(name);
        let index_model = params.push(model.into());
        my_postgres::sql::SqlUpdateValue::Index(index_name, Some(index_model))
    }
}

pub fn render_select_part() -> proc_macro2::TokenStream {
    quote::quote! {
        sql.push(my_postgres::sql::SelectFieldValue::Field(field_name));

        if let Some(metadata) = metadata {
            if let Some(field_name) = metadata.related_column_name{
                sql.push(my_postgres::sql::SelectFieldValue::Field(field_name));
            }
        }
    }
}

pub fn render_fn_is_none() -> proc_macro2::TokenStream {
    quote::quote! {
        fn is_none(&self) -> bool{
            false
        }
    }
}

pub fn get_default_value(
    enum_name: &Ident,
    enum_cases: &[EnumCase],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    for enum_case in enum_cases {
        if enum_case.attrs.has_attr("default_value") {
            let value = enum_case.get_case_string_value()?;

            return Ok(quote::quote!(#value));
        }
    }

    let enum_name = enum_name.to_string();

    let result = TokenStream::from_str(
        format!(
            r#"panic!("Default value is not specified for the enum '{}'");"#,
            enum_name
        )
        .as_str(),
    )
    .unwrap();

    Ok(result)
}
