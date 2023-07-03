use std::str::FromStr;

use proc_macro2::TokenStream;
use quote::quote;
use types_reader::{StructProperty, TypeName};

pub fn generate(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let type_name = TypeName::new(ast);

    let src_fields = StructProperty::read(ast)?;

    let src_fields = crate::postgres_struct_ext::filter_fields(src_fields)?;

    let mut limit = None;
    let mut offset = None;

    let mut fields = Vec::with_capacity(src_fields.len());

    for field in src_fields {
        if field.attrs.has_attr("limit") {
            limit = Some(field);
        } else if field.attrs.has_attr("offset") {
            offset = Some(field);
        } else {
            fields.push(field);
        }
    }

    let result = generate_implementation(&type_name, fields.iter(), limit, offset)?;

    Ok(quote! {
        #result
    }
    .into())
}

pub fn generate_implementation<'s>(
    type_name: &TypeName,
    fields: impl Iterator<Item = &'s StructProperty<'s>>,
    limit: Option<StructProperty>,
    offset: Option<StructProperty>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = type_name.get_type_name();

    let limit: TokenStream = if let Some(limit) = &limit {
        let name = limit.get_field_name_ident();
        quote! {
            fn get_limit(&self) -> Option<usize> {
                self.#name.into()
            }
        }
        .into()
    } else {
        quote! {
            fn get_limit(&self) -> Option<usize> {
                None
            }
        }
        .into()
    };

    let offset: TokenStream = if let Some(offset) = &offset {
        let name = offset.get_field_name_ident();
        quote! {
            fn get_offset(&self) -> Option<usize> {
                self.#name.into()
            }
        }
        .into()
    } else {
        quote! {
            fn get_offset(&self) -> Option<usize> {
                None
            }
        }
        .into()
    };

    let where_data = super::fn_fill_where::fn_fill_where(fields)?;

    let generics = if let Some(generics) = type_name.generics {
        TokenStream::from_str("<'s>").unwrap()
    } else {
        TokenStream::from_str("").unwrap()
    };

    let result = quote! {
       impl #generics my_postgres::sql_where::SqlWhereModel for #struct_name{
        fn get_where_field_name_data(&self, no: usize) -> Option<my_postgres::sql_where::WhereFieldData>{
            #where_data
        }
        #limit
        #offset
       }
    };

    Ok(result.into())
}
