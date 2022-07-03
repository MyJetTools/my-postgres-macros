extern crate proc_macro;
use proc_macro::TokenStream;

mod fn_impl_bulk_delete;
mod fn_impl_bulk_insert;
mod fn_impl_bulk_insert_or_update;
mod fn_impl_insert;
mod fn_impl_insert_or_update;
mod fn_impl_select;
mod fn_impl_update;
mod generators;

mod postgres_utils;
mod reflection;

use syn;

#[proc_macro_derive(PostgresSelectModel, attributes(db_field_name, debug_sql))]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_select::generate(&ast)
}

#[proc_macro_derive(
    PostgresInsertModel,
    attributes(db_field_name, ignore_if_null, debug_sql)
)]
pub fn postgres_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert::generate(&ast)
}

#[proc_macro_derive(
    PostgresUpdateModel,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_update::generate(&ast)
}

#[proc_macro_derive(
    PostgresInsertOrUpdateModel,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_insert_or_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert_or_update::generate(&ast)
}

#[proc_macro_derive(
    PostgresBulkInsertModel,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_bulk_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_bulk_insert::generate(&ast)
}

#[proc_macro_derive(
    PostgresBulkInsertOrUpdateModel,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_bulk_insert_or_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_bulk_insert_or_update::generate(&ast)
}

#[proc_macro_derive(
    PostgresBulkDelete,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_bulk_delete(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_bulk_delete::generate(&ast)
}
