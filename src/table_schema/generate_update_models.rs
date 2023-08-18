use std::str::FromStr;

use proc_macro2::TokenStream;
use types_reader::StructProperty;

use crate::{
    fn_impl_update::{generate_derive_model, UpdateFields},
    struct_name::StructName,
};

pub fn generate_update_models<'s>(
    fields: &'s [&'s StructProperty],
) -> Result<TokenStream, syn::Error> {
    let update_fields = UpdateFields::new_from_table_schema(fields)?;

    let mut result = Vec::new();
    for (struct_name, update_fields) in update_fields {
        let struct_name = proc_macro2::TokenStream::from_str(struct_name.as_str()).unwrap();

        let mut fields = Vec::new();
        for field in update_fields.get_update_fields() {
            let name = field.get_field_name_ident();
            let ty = field.ty.get_token_stream();
            fields.push(quote::quote! {
                pub #name: #ty,
            });
        }

        for field in update_fields.get_where_fields() {
            let name = field.get_field_name_ident();
            let ty = field.ty.get_token_stream();
            fields.push(quote::quote! {
                pub #name: #ty,
            });
        }

        result.push(quote::quote! {
            pub struct #struct_name{
                #(#fields)*
            }
        });

        let model = generate_derive_model(
            &struct_name,
            StructName::TokenStream(&struct_name),
            update_fields,
        )?;

        result.push(model);
    }

    let result = quote::quote! {
        #(#result)*
    };

    Ok(result.into())
}
