use proc_macro2::TokenStream;
use quote::quote;
use types_reader::{StructProperty, TypeName};

use crate::struct_name::StructName;

pub fn generate(ast: &syn::DeriveInput) -> Result<proc_macro::TokenStream, syn::Error> {
    let type_name = TypeName::new(ast);

    let src_fields = StructProperty::read(ast)?;

    let src_fields = crate::postgres_struct_ext::filter_fields(src_fields)?;

    let mut limit = None;
    let mut offset = None;

    let mut fields = Vec::with_capacity(src_fields.len());

    for struct_prop in src_fields {
        if struct_prop.attrs.has_attr("limit") {
            limit = Some(struct_prop);
        } else if struct_prop.attrs.has_attr("offset") {
            offset = Some(struct_prop);
        } else {
            fields.push(struct_prop);
        }
    }

    let result = generate_implementation(
        StructName::TypeName(&type_name),
        fields.iter(),
        limit,
        offset,
    )?;

    Ok(quote! {
        #result
    }
    .into())
}

pub fn generate_implementation<'s>(
    type_name: StructName<'s>,
    fields: impl Iterator<Item = &'s StructProperty<'s>>,
    limit: Option<StructProperty>,
    offset: Option<StructProperty>,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_name = type_name.get_struct_name();

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

    let generics = type_name.get_generic();

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
