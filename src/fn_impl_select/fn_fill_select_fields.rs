use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;

pub fn fn_fill_select_fields(
    fields: &[StructProperty],
) -> Result<Vec<proc_macro2::TokenStream>, syn::Error> {
    let mut result = Vec::with_capacity(fields.len() * 2);
    let mut no = 0;
    for prop in fields {
        if prop.is_line_no() {
            continue;
        }

        if no > 0 {
            result.push(quote! {
                sql.push(',');
            });
        }

        no += 1;

        if let Ok(sql) = prop.attrs.get_single_or_named_param("sql", "sql") {
            let attr_value = sql.as_str();
            result.push(quote! {
                sql.push_str(#attr_value);
            });
        } else {
            let db_field_name = prop.get_db_field_name();

            let metadata = crate::render_field_value::render_metadata(prop);

            if let PropertyType::OptionOf(sub_type) = &prop.ty {
                let type_ident = sub_type.get_token_stream();

                result.push(
                    quote! {
                        #type_ident::fill_select_part(sql, &[#db_field_name], &#metadata);
                    }
                    .into(),
                );
            } else {
                let type_ident = prop.ty.get_token_stream();
                result.push(
                    quote! {
                        #type_ident::fill_select_part(sql, &[#db_field_name], &#metadata);
                    }
                    .into(),
                );
            }
        }
    }

    Ok(result)
}
