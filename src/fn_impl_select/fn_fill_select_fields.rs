use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;

pub fn fn_fill_select_fields(
    fields: &[StructProperty],
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.len() * 2);
    let mut no = 0;
    for prop in fields {
        if prop.attrs.contains_key("line_no") {
            continue;
        }

        if no > 0 {
            result.push(quote! {
                sql.push(',');
            });
        }

        no += 1;

        if let Some(sql) = prop.attrs.get("sql") {
            if let Some(sql) = sql {
                if let Some(attr_value) = sql.get_single_param() {
                    let attr_value = attr_value.get_value_as_str();
                    result.push(quote! {
                        sql.push_str(#attr_value);
                    });
                } else {
                    return Err(syn::Error::new_spanned(
                        prop.field,
                        "#1 please specify content inside sql attribute",
                    ));
                }
            } else {
                return Err(syn::Error::new_spanned(
                    prop.field,
                    "#2 please specify content inside sql attribute",
                ));
            }
        } else {
            let db_field_name = prop.get_db_field_name()?;

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

    Ok(result)
}
