use macros_utils::AttributeParams;
use types_reader::{PropertyType, StructProperty};

use quote::quote;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_where(
    struct_properties: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut no = 0;

    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    lines.push(quote!(let no = 0;));

    for struct_property in struct_properties {
        let prop_name_ident = struct_property.get_field_name_ident();
        let sql_type = crate::get_field_value::fill_sql_type(struct_property);

        let op = fill_op(struct_property)?;

        let db_field_name = match struct_property.get_db_field_name() {
            Ok(result) => result,
            Err(err) => {
                return Err(syn::Error::new_spanned(struct_property.field, err));
            }
        };

        if let PropertyType::OptionOf(_) = &struct_property.ty {
            lines.push(quote! {
                if let Some(value) = self.#prop_name_ident{
                    #op
                    value.write(sql, params, #sql_type);
                }
            });
        } else {
            lines.push(quote! {
                #op
                self.#prop_name_ident.write(sql, params, #sql_type);
            });
        }

        if no > 0 {
            let name = format!(" AND {}", db_field_name);
            lines.push(quote! {
                sql.push_str(#name);
            });
        } else {
            lines.push(quote! {
                sql.push_str(#db_field_name);
            });
        }

        no += 1;
    }

    Ok(quote! {
        use my_postgres::SqlValueWriter;
        #(#lines)*
    })
}

fn fill_op(struct_property: &StructProperty) -> Result<proc_macro2::TokenStream, syn::Error> {
    let prop_name_ident = struct_property.get_field_name_ident();
    //sql.push_str(self.#prop_name_ident.get_default_operator());

    if let Some(params) = struct_property.attrs.get("operator") {
        if let Some(params) = params {
            let op = extract_and_verify_operation(params, struct_property)?;
            return Ok(quote! {
                sql.push_str(#op);
            }
            .into());
        } else {
            return Ok(quote! {
                sql.push_str(self.#prop_name_ident.get_default_operator());
            }
            .into());
        }
    } else {
        return Ok(quote! {
            sql.push_str(self.#prop_name_ident.get_default_operator());
        }
        .into());
    }
}

fn extract_and_verify_operation(
    params: &AttributeParams,
    prop: &StructProperty,
) -> Result<String, syn::Error> {
    let result = params.get_single_param();

    if result.is_none() {
        return Err(syn::Error::new_spanned(
            prop.field,
            "Operator must have a value".to_string(),
        ));
    }

    let result = result.as_ref().unwrap().get_value_as_str();

    if result == "="
        || result == "!="
        || result == "<"
        || result == "<="
        || result == ">"
        || result == ">="
        || result == "<>"
    {
        return Ok(result.to_string());
    }

    return Err(syn::Error::new_spanned(
        prop.field,
        format!("Invalid operator {}", result),
    ));
}
