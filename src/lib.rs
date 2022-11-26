extern crate proc_macro;
use db_enum::EnumType;
use proc_macro::TokenStream;

mod fn_impl_bulk_select;
mod fn_impl_insert;
mod fn_impl_select;
mod fn_impl_update;
mod fn_impl_where_data;
mod get_field_value;

mod db_enum;

mod postgres_utils;

use syn;

#[proc_macro_derive(
    SelectDbEntity,
    attributes(
        db_field_name,
        debug_sql,
        json,
        timestamp,
        bigint,
        line_no,
        sql,
        order_by,
        order_by_desc,
        group_by
    )
)]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_select::generate(&ast)
}

#[proc_macro_derive(
    BulkSelectDbEntity,
    attributes(db_field_name, debug_sql, json, timestamp, bigint, line_no)
)]
pub fn postgres_bulk_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_bulk_select::generate(&ast)
}

#[proc_macro_derive(
    WhereInputData,
    attributes(db_field_name, timestamp, bigint, operator, ignore_if_null, ignore)
)]
pub fn postgres_bulk_select_input_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_where_data::generate(&ast)
}

#[proc_macro_derive(
    InsertDbEntity,
    attributes(
        db_field_name,
        ignore_if_null,
        ignore,
        debug_sql,
        timestamp,
        bigint,
        json
    )
)]
pub fn postgres_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert::generate(&ast)
}

#[proc_macro_derive(
    UpdateDbEntity,
    attributes(db_field_name, primary_key, ignore_if_null, ignore, debug_sql)
)]
pub fn postgres_update_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_update::generate(&ast)
}

#[proc_macro_derive(DbEnumAsU8, attributes(enum_case))]
pub fn db_enum_as_u8(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::U8)
}

#[proc_macro_derive(DbEnumAsI8, attributes(enum_case))]
pub fn db_enum_as_i8(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::I8)
}

#[proc_macro_derive(DbEnumAsU16, attributes(enum_case))]
pub fn db_enum_as_u16(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::U16)
}

#[proc_macro_derive(DbEnumAsI16, attributes(enum_case))]
pub fn db_enum_as_i16(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::I16)
}

#[proc_macro_derive(DbEnumAsU32, attributes(enum_case))]
pub fn db_enum_as_u32(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::U32)
}

#[proc_macro_derive(DbEnumAsI32, attributes(enum_case))]
pub fn db_enum_as_i32(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::I32)
}

#[proc_macro_derive(DbEnumAsU64, attributes(enum_case))]
pub fn db_enum_as_u64(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::U64)
}

#[proc_macro_derive(DbEnumAsI64, attributes(enum_case))]
pub fn db_enum_as_i64(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate(&ast, EnumType::I64)
}
