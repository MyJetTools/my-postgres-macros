extern crate proc_macro;
use proc_macro::TokenStream;

mod fn_impl_delete;
mod fn_impl_insert;
mod fn_impl_insert_or_update;
mod fn_impl_select;
mod fn_impl_update;

mod db_enum;

mod postgres_utils;

use syn;

#[proc_macro_derive(
    SelectDbEntity,
    attributes(db_field_name, debug_sql, json, timestamp, bigint)
)]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_select::generate(&ast)
}

#[proc_macro_derive(
    InsertDbEntity,
    attributes(db_field_name, ignore_if_null, debug_sql, timestamp, bigint, json)
)]
pub fn postgres_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert::generate(&ast)
}

#[proc_macro_derive(
    UpdateDbEntity,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_update::generate(&ast)
}

#[proc_macro_derive(
    InsertOrUpdateDbEntity,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn postgres_insert_or_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert_or_update::generate(&ast)
}

#[proc_macro_derive(
    DeleteDbEntity,
    attributes(db_field_name, primary_key, ignore_if_null, debug_sql)
)]
pub fn delete_db_entity(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_delete::generate(&ast)
}

#[proc_macro_derive(DbEnumAsU8, attributes(enum_case))]
pub fn db_enum_as_u8(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "u8")
}

#[proc_macro_derive(DbEnumAsI8, attributes(enum_case))]
pub fn db_enum_as_i8(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "i8")
}

#[proc_macro_derive(DbEnumAsU16, attributes(enum_case))]
pub fn db_enum_as_u16(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "u16")
}

#[proc_macro_derive(DbEnumAsI16, attributes(enum_case))]
pub fn db_enum_as_i16(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "i16")
}

#[proc_macro_derive(DbEnumAsU32, attributes(enum_case))]
pub fn db_enum_as_u32(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "u32")
}

#[proc_macro_derive(DbEnumAsI32, attributes(enum_case))]
pub fn db_enum_as_i32(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "i32")
}

#[proc_macro_derive(DbEnumAsU64, attributes(enum_case))]
pub fn db_enum_as_u64(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "u64")
}

#[proc_macro_derive(DbEnumAsI64, attributes(enum_case))]
pub fn db_enum_as_i64(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, "i64")
}
