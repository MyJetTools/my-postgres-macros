use macros_utils::{AttributeParams, ParamValue};
use proc_macro2::TokenStream;
use types_reader::{PropertyType, StructProperty};

use quote::quote;

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn fn_fill_where(
    struct_properties: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut no = 0;

    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    lines.push(quote!(let mut no = 0;));

    for struct_property in struct_properties {
        let prop_name_ident = struct_property.get_field_name_ident();
        let sql_type = crate::get_field_value::fill_sql_type(struct_property);

        let db_field_name = match struct_property.get_db_field_name() {
            Ok(result) => result,
            Err(err) => {
                return Err(syn::Error::new_spanned(struct_property.field, err));
            }
        };

        if no > 0 {
            lines.push(quote! {
                if no > 0{
                    sql.push_str(" AND ");
                }
            });
        }

        if let PropertyType::OptionOf(_) = &struct_property.ty {
            let op = fill_op(quote!(value), struct_property)?;

            lines.push(quote! {
                if let Some(value) = self.#prop_name_ident{
                    sql.push_str(#db_field_name);
                    #op
                    value.write(sql, params, #sql_type);
                    no+=1;
                }
            });
        } else {
            let op = fill_op(quote!(self.#prop_name_ident), struct_property)?;
            lines.push(quote! {
                sql.push_str(#db_field_name);
                #op
                self.#prop_name_ident.write(sql, params, #sql_type);
                no+=1;
            });
        }

        no += 1;
    }

    let result = quote! {
        use my_postgres::SqlValueWriter;
        #(#lines)*
    };

    Ok(result)
}

fn fill_op(
    property: TokenStream,
    struct_property: &StructProperty,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let Some(params) = struct_property.attrs.get("operator") {
        if let Some(params) = params {
            let op_value = extract_and_verify_operation(params, struct_property)?;
            let op = op_value.get_value_as_str();

            return Ok(quote! {
                sql.push_str(#op);
            }
            .into());
        } else {
            return Ok(quote! {
                sql.push_str(#property.get_default_operator());
            }
            .into());
        }
    } else {
        return Ok(quote! {
            sql.push_str(#property.get_default_operator());
        }
        .into());
    }
}

fn extract_and_verify_operation<'s>(
    params: &'s AttributeParams,
    prop: &'s StructProperty,
) -> Result<ParamValue<'s>, syn::Error> {
    let result = params.get_single_param();

    if result.is_none() {
        return Err(syn::Error::new_spanned(
            prop.field,
            "Operator must have a value".to_string(),
        ));
    }

    let result = result.unwrap();

    if result.get_value_as_str() == "="
        || result.get_value_as_str() == "!="
        || result.get_value_as_str() == "<"
        || result.get_value_as_str() == "<="
        || result.get_value_as_str() == ">"
        || result.get_value_as_str() == ">="
        || result.get_value_as_str() == "<>"
    {
        return Ok(result);
    }

    return Err(syn::Error::new_spanned(
        prop.field,
        format!("Invalid operator {}", result.get_value_as_str()),
    ));
}
