use std::{collections::HashMap, str::FromStr};

use proc_macro2::TokenStream;
use types_reader::StructProperty;

use crate::{
    fn_impl_update::{generate_derive_model, UpdateFields},
    postgres_struct_ext::PostgresStructPropertyExt,
    struct_name::StructName,
};

pub fn generate_select_models<'s>(
    fields: &'s [&'s StructProperty],
) -> Result<TokenStream, syn::Error> {
    let mut found_fields = HashMap::new();

    for field in fields {
        let where_models = field.get_generate_additional_select_models()?;

        if let Some(where_models) = where_models {
            for where_model in where_models {
                if !found_fields.contains_key(where_model.struct_name.as_str()) {
                    found_fields.insert(where_model.struct_name.to_string(), Vec::new());
                }

                found_fields
                    .get_mut(where_model.struct_name.as_str())
                    .unwrap()
                    .push((where_model, field));
            }
        }
    }

    let mut result = Vec::new();

    for (struct_name, models) in found_fields {
        let struct_name = TokenStream::from_str(struct_name.as_str()).unwrap();

        let mut fields = Vec::new();

        for (model, field) in models {
            let field_name = TokenStream::from_str(model.field_name.as_str()).unwrap();
            let ty = &model.field_ty;

            if let Some(db_column_name) = field.try_get_db_column_name_as_string()? {
                super::attr_generators::generate_db_column_name_attribute(
                    &mut fields,
                    db_column_name,
                );
            }

            if let Some(sql_type) = field.try_get_sql_type() {
                let sql_type = sql_type.unwrap_as_string_value()?;
                let sql_type = sql_type.as_str();
                super::attr_generators::generate_sql_type(&mut fields, sql_type);
            }

            fields.push(quote::quote! {
                pub #field_name: #ty,
            });
        }

        result.push(quote::quote! {
            #[derive(my_postgres_macros::SelectDbEntity)]
            pub struct #struct_name{
                #(#fields)*
            }
        });
    }

    let result = quote::quote! {
        #(#result)*
    };

    Ok(result)
}
