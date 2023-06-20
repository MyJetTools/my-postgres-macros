use my_postgres_macros::{
    DbEnumAsString, DbEnumAsStringWithModel, DbEnumAsU16WithModel, InsertDbEntity,
    MyPostgresJsonModel, SelectDbEntity, TableSchema, UpdateDbEntity,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct TestJsonField {
    #[primary_key]
    pub id: i32,

    #[sql_type("timestamp")]
    pub date: DateTimeAsMicroseconds,

    #[db_field_name("myTest")]
    pub test: MyStructure,

    pub my_enum_no_model: MyEnum,

    #[db_field_name(model_field_name = "field_model")]
    pub my_enum: MyEnumWithModel,

    pub b: bool,

    pub c: f32,
    pub d: f64,
}

#[derive(Serialize, Deserialize, MyPostgresJsonModel)]
pub struct MyStructure {
    pub a: i32,
    pub b: bool,
}

#[derive(Serialize, Deserialize, DbEnumAsString)]
pub enum MyEnum {
    #[enum_case("Test")]
    Case1,
    #[enum_case("Test2")]
    Case2,
}

#[derive(Serialize, Deserialize, DbEnumAsStringWithModel)]
pub enum MyEnumWithModel {
    #[enum_case("Test")]
    Case1(MyStructure),
    #[enum_case("Test2")]
    Case2(MyStructure),
}

#[derive(Serialize, Deserialize, DbEnumAsU16WithModel)]
pub enum MyEnumWordWithModel {
    #[enum_case(0)]
    Case1(MyStructure),
    #[enum_case(1)]
    Case2(MyStructure),
}
