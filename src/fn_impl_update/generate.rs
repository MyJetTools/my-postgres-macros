use proc_macro::TokenStream;
use types_reader::{StructProperty, TypeName};

use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let type_name = TypeName::new(ast);

    let struct_name = type_name.get_type_name();

    let life_time = type_name.get_default_lifetime_generic();

    let fields = StructProperty::read(ast);

    let mut primary_key_amount = 0;

    for field in &fields {
        if field.is_primary_key() {
            primary_key_amount += 1;
        }
    }

    let get_field_value_case = super::fn_get_field_value::fn_get_field_value(fields.as_slice());

    let fields_ammount = fields.len() - primary_key_amount;

    let mut with_primary_key = Vec::new();

    for field in fields {
        if field.is_primary_key() {
            with_primary_key.push(field);
        }
    }

    let where_impl = crate::fn_impl_where_model::generate_implementation(
        &type_name,
        with_primary_key.as_slice(),
        None,
        None,
    );

    quote! {

        impl #life_time my_postgres::sql_update::SqlUpdateModel #life_time for #struct_name{
            fn get_fields_amount() -> usize{
                #fields_ammount
            }
            fn get_field_value(& #life_time self, no: usize) -> my_postgres::sql_update::SqlUpdateValue #life_time{
                match no{
                    #(#get_field_value_case)*
                    _=>panic!("no such field with number {}", no)
                }

            }
        }

        #where_impl
    }
    .into()
}
