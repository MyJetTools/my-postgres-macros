extern crate proc_macro;
use proc_macro::TokenStream;

mod fn_impl_insert;
mod fn_impl_insert_or_update;
mod fn_impl_reading;
mod fn_impl_update;
mod generators;

mod postgres_utils;
mod reflection;

use syn;

#[proc_macro_derive(PostgresSelectModel, attributes(db_field_name))]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_reading::generate(&ast)
}

#[proc_macro_derive(PostgresInsertModel, attributes(db_field_name, ignore_if_null))]
pub fn postgres_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert::generate(&ast)
}

#[proc_macro_derive(
    PostgresUpdateModel,
    attributes(db_field_name, primary_key, ignore_if_null)
)]
pub fn postgres_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_update::generate(&ast)
}

#[proc_macro_derive(
    PostgresInsertOrUpdateModel,
    attributes(db_field_name, primary_key, ignore_if_null)
)]
pub fn postgres_insert_or_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert_or_update::generate(&ast)
}
