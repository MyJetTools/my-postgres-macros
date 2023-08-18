use std::{collections::HashMap, str::FromStr};

use proc_macro2::TokenStream;
use types_reader::StructProperty;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn generate_where_models<'s>(
    fields: &'s [&'s StructProperty],
) -> Result<TokenStream, syn::Error> {
    let mut found_fields = HashMap::new();

    for field in fields {
        let where_models = field.get_generate_additional_where_model()?;

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
            let ty = model.field_ty;

            if let Some(operator_from) = model.operator_from.as_ref() {
                fields.push(quote::quote! {
                    #[operator(#operator_from)]
                });

                let field_name =
                    TokenStream::from_str(format!("{}_from", model.field_name.as_str()).as_str())
                        .unwrap();

                generate_additional_attributes(&mut fields, field)?;

                let db_column_name = field.get_db_column_name_as_string()?;

                generate_db_column_name_attribute(&mut fields, db_column_name);

                fields.push(quote::quote! {
                    #field_name: #ty,
                });

                if let Some(operator_to) = model.operator_to.as_ref() {
                    fields.push(quote::quote! {
                        #[operator(#operator_to)]
                    });

                    let field_name =
                        TokenStream::from_str(format!("{}_to", model.field_name.as_str()).as_str())
                            .unwrap();

                    generate_additional_attributes(&mut fields, field)?;

                    generate_db_column_name_attribute(&mut fields, db_column_name);

                    fields.push(quote::quote! {
                        #field_name: #ty,
                    });
                }
            } else {
                let field_name = TokenStream::from_str(model.field_name.as_str()).unwrap();

                if let Some(operator) = model.operator.as_ref() {
                    fields.push(quote::quote! {
                        #[operator(#operator)]
                    })
                }

                if let Some(db_column_name) = field.try_get_db_column_name_as_string()? {
                    generate_db_column_name_attribute(&mut fields, db_column_name);
                }

                generate_additional_attributes(&mut fields, field)?;

                fields.push(quote::quote! {
                    #field_name: #ty,
                });
            }
        }

        result.push(quote::quote! {
            #[derive(my_postgres_macros::WhereDbModel)]
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

fn generate_additional_attributes(
    fields: &mut Vec<TokenStream>,
    field: &StructProperty,
) -> Result<(), syn::Error> {
    if let Some(sql_type) = field.try_get_sql_type() {
        let sql_type = sql_type.unwrap_as_string_value()?;
        let sql_type = sql_type.as_str();
        fields.push(quote::quote! {
            #[sql_type(#sql_type)]
        })
    }

    Ok(())
}

fn generate_db_column_name_attribute(fields: &mut Vec<TokenStream>, db_column_name: &str) {
    fields.push(quote::quote! {
        #[db_column_name(#db_column_name)]
    })
}
