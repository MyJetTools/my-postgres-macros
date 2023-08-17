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

#[cfg(test)]
mod tests {

    use my_postgres::sql::UsedColumns;

    use super::*;

    fn hash_maps_are_equal(src: &HashMap<String, String>, dest: &HashMap<String, String>) {
        assert_eq!(src.len(), dest.len());
        for (key, value) in src.iter() {
            assert_eq!(value, dest.get(key).unwrap());
        }
    }

    #[test]
    fn test_insert_case_opt_fields_are_null() {
        let mut field = HashMap::new();
        field.insert("key1".to_string(), "value1".to_string());
        field.insert("key2".to_string(), "value2".to_string());

        let my_dt =
            DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap();

        let dto = JsonHashMapDto {
            field,
            opt_field: None,
            field_vec: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            opt_vec: None,
            json_field: MyJsonStruct {
                data: "test".to_string(),
            },
            opt_json_field: None,
            my_dt,
        };

        let sql = my_postgres::sql::build_insert_sql(
            &dto,
            "test_table_name",
            &mut UsedColumns::as_none(),
        );

        assert_eq!(sql.sql,
        "INSERT INTO test_table_name(field,opt_field,field_vec,opt_vec,json_field,opt_json_field,my_dt) VALUES (cast($1::text as json),NULL,cast($2::text as json),NULL,cast($3::text as json),NULL,'2023-06-19T22:07:20.518741+00:00')");

        assert_eq!(3, sql.values.len());

        let result: HashMap<String, String> =
            serde_json::from_str(sql.values.get(0).unwrap().as_str()).unwrap();

        hash_maps_are_equal(&dto.field, &result);

        assert_eq!("[\"1\",\"2\",\"3\"]", sql.values.get(1).unwrap().as_str());

        assert_eq!("{\"data\":\"test\"}", sql.values.get(2).unwrap().as_str());
    }

    #[test]
    fn test_insert_case_opt_fields_are_not_null() {
        let mut field = HashMap::new();
        field.insert("key1".to_string(), "value1".to_string());
        field.insert("key2".to_string(), "value2".to_string());

        let my_dt =
            DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap();

        let dto = JsonHashMapDto {
            field: field.clone(),
            opt_field: field.into(),
            field_vec: vec!["1".to_string(), "2".to_string(), "3".to_string()],
            opt_vec: vec!["3".to_string(), "4".to_string(), "5".to_string()].into(),
            json_field: MyJsonStruct {
                data: "test".to_string(),
            },
            opt_json_field: MyJsonStruct {
                data: "test2".to_string(),
            }
            .into(),
            my_dt,
        };

        let sql = my_postgres::sql::build_insert_sql(
            &dto,
            "test_table_name",
            &mut UsedColumns::as_none(),
        );

        assert_eq!(sql.sql,
        "INSERT INTO test_table_name(field,opt_field,field_vec,opt_vec,json_field,opt_json_field,my_dt) VALUES (cast($1::text as json),cast($1::text as json),cast($2::text as json),cast($3::text as json),cast($4::text as json),cast($5::text as json),'2023-06-19T22:07:20.518741+00:00')");

        assert_eq!(5, sql.values.len());

        let result: HashMap<String, String> =
            serde_json::from_str(sql.values.get(0).unwrap().as_str()).unwrap();

        hash_maps_are_equal(&dto.field, &result);

        assert_eq!("[\"1\",\"2\",\"3\"]", sql.values.get(1).unwrap().as_str());
        assert_eq!("[\"3\",\"4\",\"5\"]", sql.values.get(2).unwrap().as_str());
        assert_eq!("{\"data\":\"test\"}", sql.values.get(3).unwrap().as_str());
        assert_eq!("{\"data\":\"test2\"}", sql.values.get(4).unwrap().as_str());
    }
}
