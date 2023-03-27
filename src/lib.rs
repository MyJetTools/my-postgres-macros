extern crate proc_macro;
use db_enum::EnumType;
use proc_macro::TokenStream;

mod fn_impl_bulk_select;
mod fn_impl_insert;
mod fn_impl_select;
mod fn_impl_update;
mod fn_impl_where_model;
mod render_field_value;
mod table_schema;

mod db_enum;

mod my_postgres_json_model;

mod postgres_enum_ext;
mod postgres_struct_ext;

use syn;

#[proc_macro_derive(
    SelectDbEntity,
    attributes(
        db_field_name,
        line_no,
        sql,
        sql_type,
        order_by,
        order_by_desc,
        group_by,
        primary_key
    )
)]
pub fn postgres_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_select::generate(&ast)
}

#[proc_macro_derive(
    BulkSelectDbEntity,
    attributes(db_field_name, json, bigint, line_no, sql_type,)
)]
pub fn postgres_bulk_select_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_bulk_select::generate(&ast)
}

#[proc_macro_derive(
    WhereDbModel,
    attributes(
        db_field_name,
        bigint,
        operator,
        ignore_if_null,
        ignore,
        limit,
        offset,
        sql_type,
    )
)]
pub fn postgres_bulk_select_input_data(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_where_model::generate(&ast)
}

#[proc_macro_derive(
    InsertDbEntity,
    attributes(
        db_field_name,
        ignore_if_null,
        ignore,
        bigint,
        json,
        sql_type,
        primary_key
    )
)]
pub fn postgres_insert_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::fn_impl_insert::generate(&ast)
}

#[proc_macro_derive(
    UpdateDbEntity,
    attributes(db_field_name, primary_key, ignore_if_null, ignore, sql_type,)
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

#[proc_macro_derive(DbEnumAsString, attributes(enum_case))]
pub fn db_enum_as_string(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_as_string(&ast)
}

#[proc_macro_derive(DbEnumAsU8WithModel, attributes(enum_case))]
pub fn db_enum_as_u8_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::U8)
}

#[proc_macro_derive(DbEnumAsI8WithModel, attributes(enum_case))]
pub fn db_enum_as_i8_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::I8)
}

#[proc_macro_derive(DbEnumAsU16WithModel, attributes(enum_case))]
pub fn db_enum_as_u16_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::U16)
}

#[proc_macro_derive(DbEnumAsI16WithModel, attributes(enum_case))]
pub fn db_enum_as_i16_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::I16)
}

#[proc_macro_derive(DbEnumAsU32WithModel, attributes(enum_case))]
pub fn db_enum_as_u32_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::U32)
}

#[proc_macro_derive(DbEnumAsI32WithModel, attributes(enum_case))]
pub fn db_enum_as_i32_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::I32)
}

#[proc_macro_derive(DbEnumAsU64WithModel, attributes(enum_case))]
pub fn db_enum_as_u64_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::U64)
}

#[proc_macro_derive(DbEnumAsI64WithModel, attributes(enum_case))]
pub fn db_enum_as_i64_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_with_model(&ast, EnumType::I64)
}

#[proc_macro_derive(DbEnumAsStringWithModel, attributes(enum_case))]
pub fn db_enum_as_string_with_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::db_enum::generate_as_string_with_model(&ast)
}

#[proc_macro_derive(MyPostgresJsonModel, attributes(enum_case))]
pub fn my_postgres_json_model(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    crate::my_postgres_json_model::generate(&ast)
}

#[proc_macro_derive(
    TableSchema,
    attributes(
        db_field_name,
        bigint,
        operator,
        ignore_if_null,
        ignore,
        limit,
        offset,
        sql_type,
        ignore_table_column,
        primary_key,
        db_index,
    )
)]
pub fn table_schema(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    let result = crate::table_schema::generate(&ast);

    #[cfg(feature = "debug-table-schema")]
    println!("{}", result);
    result
}
