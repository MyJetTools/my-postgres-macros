use proc_macro2::TokenStream;
use quote::quote;
use types_reader::StructProperty;

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name: TokenStream = {
        let ident = &ast.ident;

        if ast.generics.lifetimes().count() > 0 {
            let generics = &ast.generics;
            quote!(#ident #generics)
        } else {
            quote!(#ident)
        }
    };

    let src_fields = StructProperty::read(ast);

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

    let result = generate_implementation(&struct_name, fields.as_slice(), limit, offset);

    quote! {
        #result
    }
    .into()
}

pub fn generate_implementation(
    struct_name: &TokenStream,
    fields: &[StructProperty],
    limit: Option<StructProperty>,
    offset: Option<StructProperty>,
) -> proc_macro2::TokenStream {
    let limit: TokenStream = if let Some(limit) = &limit {
        let name = limit.get_field_name_ident();
        quote! {
            fn get_limit(&self) -> Option<usize> {
                self.#name.as_str()
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
                self.#name.as_str()
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

    let where_data = super::fn_fill_where::fn_fill_where(fields);
    quote! {
       impl<'s> my_postgres::sql_where::SqlWhereModel<'s> for #struct_name{
        fn fill_where(&'s self, sql: &mut String, params: &mut Vec<&'s (dyn tokio_postgres::types::ToSql + Sync)>,) {
            #where_data
        }
        #limit
        #offset
       }
    }
    .into()
}
