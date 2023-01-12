use proc_macro2::TokenStream;
use quote::quote;
use types_reader::StructProperty;

use crate::postgres_struct_ext::PostgresStructPropertyExt;

pub fn render_field_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    match &struct_propery.ty {
        types_reader::PropertyType::OptionOf(_) => return fill_option_of_value(struct_propery),
        types_reader::PropertyType::Struct(..) => return get_value(struct_propery),
        _ => return get_value(struct_propery),
    }
}

fn get_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    let name = struct_propery.get_field_name_ident();

    let metadata = struct_propery.get_field_metadata();

    quote! {
        my_postgres::SqlValueWrapper::Value {
            value: &self.#name,
            metadata: #metadata
        }
    }
    .into()
}

fn fill_option_of_value(struct_propery: &StructProperty) -> proc_macro2::TokenStream {
    let prop_name = struct_propery.get_field_name_ident();

    let metadata = struct_propery.get_field_metadata();

    let else_case: TokenStream = if struct_propery.has_ignore_if_null_attr() {
        quote!(my_postgres::SqlValueWrapper::Ignore).into()
    } else {
        quote!(my_postgres::SqlValueWrapper::Null).into()
    };

    quote! {
       if let Some(value) = &self.#prop_name{
          my_postgres::SqlValueWrapper::Value {value, metadata: #metadata}
       }else{
            #else_case
       }
    }
}
