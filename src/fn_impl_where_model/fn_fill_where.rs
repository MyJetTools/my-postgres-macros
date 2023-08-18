use types_reader::{ParamValue, StructProperty};

use quote::quote;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn fn_fill_where<'s>(
    struct_properties: impl Iterator<Item = &'s StructProperty<'s>>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut no: usize = 0;

    for struct_property in struct_properties {
        let prop_name_ident = struct_property.get_field_name_ident();
        let metadata = struct_property.get_field_metadata()?;

        let db_column_name = struct_property.get_db_column_name_as_string()?;

        let op = fill_op(struct_property)?;

        let ignore_if_none = struct_property.has_ignore_if_none_attr();

        let value = if struct_property.ty.is_option() {
            quote! {
                if let Some(value) = &self.#prop_name_ident{
                    Some(value)
                }else{
                    None
                }

            }
        } else {
            quote! {
                Some(&self.#prop_name_ident)
            }
        };

        lines.push(quote! {
           #no => Some(WhereFieldData{
                column_name: #db_column_name.into(),
                op: #op,
                value: #value,
                ignore_if_none: #ignore_if_none,
                meta_data: #metadata
            }),
        });

        no += 1;
    }

    lines.push(quote::quote!(_ => None));

    let result = quote! {
        use my_postgres::sql_where::WhereFieldData;
        match no{
            #(#lines)*,
        }

    };

    Ok(result)
}

fn fill_op(struct_property: &StructProperty) -> Result<proc_macro2::TokenStream, syn::Error> {
    if let Ok(op_value) = struct_property
        .attrs
        .get_single_or_named_param("operator", "op")
    {
        let op_value = extract_and_verify_operation(op_value, struct_property)?;
        let op = op_value.unwrap_as_string_value()?.as_str();

        if op == "like" || op == "LIKE" {
            return Ok(quote! {
                Some(" like ")
            }
            .into());
        }

        return Ok(quote! {
            Some(#op)
        }
        .into());
    } else {
        return Ok(quote! {
        None
         }
        .into());
    }
}

fn extract_and_verify_operation<'s>(
    op_value: &'s ParamValue,
    prop: &'s StructProperty,
) -> Result<&'s ParamValue, syn::Error> {
    let value = op_value.unwrap_as_string_value()?.as_str();
    if value == "="
        || value == "!="
        || value == "<"
        || value == "<="
        || value == ">"
        || value == ">="
        || value == "<>"
        || value == "like"
        || value == "LIKE"
    {
        return Ok(op_value);
    }

    return Err(syn::Error::new_spanned(
        prop.field,
        format!("Invalid operator {}", value),
    ));
}
