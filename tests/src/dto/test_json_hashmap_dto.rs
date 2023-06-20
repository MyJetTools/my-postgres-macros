use std::collections::HashMap;

use my_postgres_macros::{InsertDbEntity, SelectDbEntity, UpdateDbEntity};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity)]
pub struct JsonHashMapDto {
    pub field: HashMap<String, String>,
    pub opt_field: Option<HashMap<String, String>>,
    pub field_vec: Vec<String>,
    pub opt_vec: Option<Vec<String>>,
    pub json_field: MyJsonStruct,
    pub opt_json_field: Option<MyJsonStruct>,

    #[sql_type("timestamp")]
    pub my_dt: DateTimeAsMicroseconds,
}

#[derive(serde::Serialize, serde::Deserialize, my_postgres_macros::MyPostgresJsonModel)]
pub struct MyJsonStruct {
    pub data: String,
}
