use types_reader::{ParamValue, StructProperty};

use quote::quote;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn fn_fill_where(
    struct_properties: &[StructProperty],
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let mut lines: Vec<proc_macro2::TokenStream> = Vec::new();

    let mut no: usize = 0;

    for struct_property in struct_properties {
        let prop_name_ident = struct_property.get_field_name_ident();
        let metadata = struct_property.get_field_metadata()?;

        let db_field_name = struct_property.get_db_field_name_as_string()?;

        let op = fill_op(struct_property)?;

        let ignore_if_none = struct_property.has_ignore_if_none_attr();

        lines.push(quote! {
           #no => Some(WhereFieldData{
                field_name: #db_field_name,
                op: #op,
                value: &self.#prop_name_ident,
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
    {
        return Ok(op_value);
    }

    return Err(syn::Error::new_spanned(
        prop.field,
        format!("Invalid operator {}", value),
    ));
}
