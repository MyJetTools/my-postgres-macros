use proc_macro2::TokenStream;
use quote::quote;
use types_reader::StructProperty;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn render_field_value(
    struct_propery: &StructProperty,
    is_update: bool,
) -> proc_macro2::TokenStream {
    match &struct_propery.ty {
        types_reader::PropertyType::OptionOf(_) => {
            return fill_option_of_value(struct_propery, is_update)
        }
        types_reader::PropertyType::Struct(..) => return get_value(struct_propery, is_update),
        _ => return get_value(struct_propery, is_update),
    }
}

fn get_value(struct_propery: &StructProperty, is_update: bool) -> proc_macro2::TokenStream {
    let name = struct_propery.get_field_name_ident();

    let metadata = struct_propery.get_field_metadata();

    if is_update {
        quote! {
            my_postgres::SqlUpdateValueWrapper::Value {
                value: &self.#name,
                metadata: #metadata
            }
        }
        .into()
    } else {
        quote! {
            my_postgres::SqlWhereValueWrapper::Value {
                value: &self.#name,
                metadata: #metadata
            }
        }
        .into()
    }
}

fn fill_option_of_value(
    struct_propery: &StructProperty,
    is_update: bool,
) -> proc_macro2::TokenStream {
    let prop_name = struct_propery.get_field_name_ident();

    let metadata = struct_propery.get_field_metadata();

    let else_case: TokenStream = if struct_propery.has_ignore_if_null_attr() {
        if is_update {
            quote!(my_postgres::SqlUpdateValueWrapper::Ignore).into()
        } else {
            quote!(my_postgres::SqlWhereValueWrapper::Ignore).into()
        }
    } else {
        if is_update {
            quote!(my_postgres::SqlUpdateValueWrapper::Null).into()
        } else {
            quote!(my_postgres::SqlWhereValueWrapper::Null).into()
        }
    };

    if is_update {
        quote! {
           if let Some(value) = &self.#prop_name{
              my_postgres::SqlUpdateValueWrapper::Value {value, metadata: #metadata}
           }else{
                #else_case
           }
        }
    } else {
        quote! {
           if let Some(value) = &self.#prop_name{
              my_postgres::SqlWhereValueWrapper::Value {value, metadata: #metadata}
           }else{
                #else_case
           }
        }
    }
}
