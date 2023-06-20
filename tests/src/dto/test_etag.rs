use my_postgres_macros::{InsertDbEntity, SelectDbEntity, TableSchema, UpdateDbEntity};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct TestETagDto {
    #[primary_key]
    pub id: i32,

    #[sql_type("timestamp")]
    pub date: DateTimeAsMicroseconds,

    #[db_field_name("etag")]
    #[e_tag]
    pub e_tag: i64,
}
