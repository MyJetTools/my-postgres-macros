use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;

pub fn fn_fill_select_fields(fields: &[StructProperty]) -> Vec<proc_macro2::TokenStream> {
    let mut result = Vec::with_capacity(fields.len() * 2);
    let mut no = 0;
    for prop in fields {
        if prop.attrs.has_attr("line_no") {
            continue;
        }

        if no > 0 {
            result.push(quote! {
                sql.push(',');
            });
        }

        no += 1;

        if let Some(sql) = prop.attrs.try_get("sql") {
            if let Some(value) = &sql.content {
                let attr_value = crate::postgres_utils::extract_attr_value(value);

                result.push(quote! {
                    sql.push_str(#attr_value);
                });
            } else {
                panic!(
                    "please specify content inside sql attribute for {}",
                    prop.name
                );
            }
        } else {
            let db_field_name = prop.get_db_field_name();

            let sql_type = super::fill_sql_type(prop);

            if let PropertyType::OptionOf(sub_type) = &prop.ty {
                let type_ident = sub_type.get_token_stream();

                result.push(
                    quote! {
                        #type_ident::fill_select_part(sql, #db_field_name, #sql_type);
                    }
                    .into(),
                );
            } else {
                let type_ident = prop.ty.get_token_stream();
                result.push(
                    quote! {
                        #type_ident::fill_select_part(sql, #db_field_name, #sql_type);
                    }
                    .into(),
                );
            }
        }
    }

    result
}
