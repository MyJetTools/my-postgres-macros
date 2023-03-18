use proc_macro2::TokenStream;
use quote::quote;
use types_reader::{StructProperty, TypeName};

pub fn generate(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let type_name = TypeName::new(ast);

    let src_fields = match StructProperty::read(ast) {
        Ok(fields) => fields,
        Err(e) => return e.into_compile_error().into(),
    };

    let src_fields = match crate::postgres_struct_ext::filter_fields(src_fields) {
        Ok(result) => result,
        Err(err) => return err.into(),
    };

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

    let result = generate_implementation(&type_name, fields.as_slice(), limit, offset);

    quote! {
        #result
    }
    .into()
}

pub fn generate_implementation(
    type_name: &TypeName,
    fields: &[StructProperty],
    limit: Option<StructProperty>,
    offset: Option<StructProperty>,
) -> proc_macro2::TokenStream {
    let struct_name = type_name.get_type_name();

    let limit: TokenStream = if let Some(limit) = &limit {
        println!("Rendering Limit for {}", type_name.struct_name);
        let name = limit.get_field_name_ident();
        quote! {
            fn get_limit(&self) -> Option<usize> {
                self.#name.into()
            }
        }
        .into()
    } else {
        println!("Rendering NoLimit for {}", type_name.struct_name);
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

    let where_data = match super::fn_fill_where::fn_fill_where(fields) {
        Ok(result) => result,
        Err(err) => err.to_compile_error(),
    };

    quote! {
       impl<'s> my_postgres::sql_where::SqlWhereModel<'s> for #struct_name{
        fn fill_where(&'s self, sql: &mut String, params: &mut Vec<my_postgres::SqlValue<'s>>,) {
            #where_data
        }
        #limit
        #offset
       }
    }
    .into()
}
