use proc_macro2::TokenStream;
use types_reader::{attribute_params::ParamValue, PropertyType, StructProperty};

use quote::quote;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn fn_fill_where(
    struct_properties: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut no = 0;

    lines.push(quote!(let mut no = 0;));

    for struct_property in struct_properties {
        let prop_name_ident = struct_property.get_field_name_ident();
        let metadata = struct_property.get_field_metadata();

        let db_field_name = struct_property.get_db_field_name_as_string();

        let push_and = if no > 0 {
            Some(quote! {
                if no > 0{
                    sql.push_str(" AND ");
                }
            })
        } else {
            None
        };

        if let PropertyType::OptionOf(_) = &struct_property.ty {
            let op = fill_op(quote!(value), struct_property)?;

            lines.push(quote! {
                if let Some(value) = &self.#prop_name_ident{
                    #push_and
                    sql.push_str(#db_field_name);
                    #op
                    value.write(sql, params, &#metadata);
                    no+=1;
                }
            });
        } else {
            let op = fill_op(quote!(self.#prop_name_ident), struct_property)?;
            lines.push(quote! {
                #push_and
                sql.push_str(#db_field_name);
                #op
                self.#prop_name_ident.write(sql, params, &#metadata);
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
    if let Ok(op_value) = struct_property
        .attrs
        .get_single_or_named_param("operator", "op")
    {
        let op_value = extract_and_verify_operation(op_value, struct_property)?;
        let op = op_value.as_str();

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
}

fn extract_and_verify_operation<'s>(
    op_value: ParamValue<'s>,
    prop: &'s StructProperty,
) -> Result<ParamValue<'s>, syn::Error> {
    if op_value.as_str() == "="
        || op_value.as_str() == "!="
        || op_value.as_str() == "<"
        || op_value.as_str() == "<="
        || op_value.as_str() == ">"
        || op_value.as_str() == ">="
        || op_value.as_str() == "<>"
    {
        return Ok(op_value);
    }

    return Err(syn::Error::new_spanned(
        prop.field,
        format!("Invalid operator {}", op_value.as_str()),
    ));
}
