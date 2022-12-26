use proc_macro::TokenStream;
use types_reader::StructProperty;

use crate::postgres_utils::PostgresStructPropertyExt;
use quote::quote;

pub fn generate(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;

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
        struct_name,
        with_primary_key.as_slice(),
        None,
        None,
    );

    quote! {

        impl<'s> my_postgres::sql_update::SqlUpdateModel<'s> for #struct_name{
            fn get_fields_amount() -> usize{
                #fields_ammount
            }
            fn get_field_value(&'s self, no: usize) -> my_postgres::sql_update::SqlUpdateValue<'s>{
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
