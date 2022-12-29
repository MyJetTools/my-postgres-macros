use proc_macro2::TokenStream;
use quote::quote;
use types_reader::{PropertyType, StructProperty};

use crate::postgres_utils::PostgresStructPropertyExt;

pub fn get_field_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    match &struct_propery.ty {
        types_reader::PropertyType::U8 => return get_value(struct_propery),
        types_reader::PropertyType::I8 => return get_value(struct_propery),
        types_reader::PropertyType::U16 => return get_value(struct_propery),
        types_reader::PropertyType::I16 => return get_value(struct_propery),
        types_reader::PropertyType::U32 => return get_value(struct_propery),
        types_reader::PropertyType::I32 => return get_value(struct_propery),
        types_reader::PropertyType::U64 => return get_value(struct_propery),
        types_reader::PropertyType::I64 => return get_value(struct_propery),
        types_reader::PropertyType::F32 => return get_value(struct_propery),
        types_reader::PropertyType::F64 => return get_value(struct_propery),
        types_reader::PropertyType::String => return get_value(struct_propery),
        types_reader::PropertyType::Str => return get_value(struct_propery),
        types_reader::PropertyType::DateTime => return get_value(struct_propery),
        types_reader::PropertyType::OptionOf(sub_type) => {
            return fill_option_of(struct_propery, &sub_type)
        }
        types_reader::PropertyType::Struct(..) => return get_value(struct_propery),
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn fill_option_of(
    struct_propery: &StructProperty,
    sub_type: &PropertyType,
) -> proc_macro2::TokenStream {
    match &sub_type {
        types_reader::PropertyType::U8 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::I8 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::U16 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::I16 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::U32 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::I32 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::U64 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::I64 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::F32 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::F64 => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::String => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::Str => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::DateTime => return fill_option_of_value(struct_propery),
        _ => panic!("{} is not supported", struct_propery.ty.as_str()),
    }
}

fn get_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    let name = struct_propery.get_field_name_ident();

    let metadata = render_metadata(struct_propery);

    quote! {
        my_postgres::sql_value::SqlValueWrapper::Value {
            value: &self.#name,
            metadata: #metadata
        }
    }
    .into()
}

fn fill_option_of_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    let prop_name = struct_propery.get_field_name_ident();

    let metadata = render_metadata(struct_propery);

    let else_case: TokenStream = if struct_propery.has_ignore_if_null_attr() {
        quote!(my_postgres::sql_value::SqlValueWrapper::Ignore).into()
    } else {
        quote!(my_postgres::sql_value::SqlValueWrapper::Null).into()
    };

    quote! {
       if let Some(value) = &self.#prop_name{
          my_postgres::sql_value::SqlValueWrapper::Value {value, sql_type: #metadata}
       }else{
            #else_case
       }
    }
}

pub fn render_metadata(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    if let Some(sql_type) = struct_propery.get_sql_type() {
        return quote! {
            Some(#sql_type)
        }
        .into();
    }

    quote!(None).into()
}
