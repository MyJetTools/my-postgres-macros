use my_postgres_macros::{DbEnumAsString, WhereDbModel};
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(WhereDbModel)]
pub struct TestWhereModel {
    #[db_field_name("id")]
    pub name: String,

    #[sql_type("timestamp")]
    pub date_time: DateTimeAsMicroseconds,

    #[operator(">")]
    pub i32: i32,

    pub opt_i32: Option<i32>,

    #[ignore_if_none]
    pub ignore_if_null: Option<i32>,

    pub str_enum: MyWhereStringEnum,

    pub str_enum_opt: Option<MyWhereStringEnum>,
    #[ignore_if_none]
    pub str_enum_opt_to_ignore: Option<MyWhereStringEnum>,

    pub str_enum_opt2: Option<MyWhereStringEnum>,

    #[limit]
    pub limit: usize,

    #[operator("like")]
    pub like_value: String,
}

#[derive(DbEnumAsString)]
pub enum MyWhereStringEnum {
    #[enum_case("Test")]
    Case1,
    #[enum_case("Test2")]
    Case2,
}

#[cfg(test)]
mod tests {
    use my_postgres::{sql::SqlValues, sql_where::SqlWhereModel};
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::TestWhereModel;

    #[test]
    fn test_generating_where_part() {
        let date_time =
            DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap();
        let where_model = TestWhereModel {
            name: "test".to_string(),
            date_time,
            i32: 1,
            opt_i32: None,
            ignore_if_null: None,
            str_enum: super::MyWhereStringEnum::Case1,
            limit: 10,
            str_enum_opt: None,
            str_enum_opt_to_ignore: None,
            str_enum_opt2: Some(super::MyWhereStringEnum::Case2),
            like_value: "test".to_string(),
        };

        let mut params = SqlValues::new();
        let where_builder: my_postgres::sql::WhereBuilder =
            where_model.build_where_sql_part(&mut params);

        let result = where_builder.get(0).unwrap();
        assert_eq!(result.db_column_name, "id");
        assert_eq!(result.op, "=");
        assert_eq!(
            params
                .get(result.value.unwrap_as_index() - 1)
                .unwrap()
                .as_str(),
            "test"
        );

        let result = where_builder.get(1).unwrap();
        assert_eq!(result.db_column_name, "date_time");
        assert_eq!(result.op, "=");
        assert_eq!(
            result.value.unwrap_as_string_value(),
            "2023-06-19T22:07:20.518741+00:00"
        );

        let result = where_builder.get(2).unwrap();
        assert_eq!(result.db_column_name, "i32");
        assert_eq!(result.op, ">");
        assert_eq!(result.value.unwrap_as_non_string_value(), "1");

        let result = where_builder.get(3).unwrap();
        assert_eq!(result.db_column_name, "opt_i32");
        assert_eq!(result.op, " IS ");
        assert_eq!(result.value.unwrap_as_non_string_value(), "NULL");

        let result = where_builder.get(4).unwrap();
        assert_eq!(result.db_column_name, "str_enum");
        assert_eq!(result.op, "=");
        assert_eq!(
            params
                .get(result.value.unwrap_as_index() - 1)
                .unwrap()
                .as_str(),
            "Test"
        );

        let result = where_builder.get(5).unwrap();
        assert_eq!(result.db_column_name, "str_enum_opt");
        assert_eq!(result.op, " IS ");
        assert_eq!(result.value.unwrap_as_non_string_value(), "NULL");

        let result = where_builder.get(6).unwrap();
        assert_eq!(result.db_column_name, "str_enum_opt2");
        assert_eq!(result.op, "=");
        assert_eq!(
            params
                .get(result.value.unwrap_as_index() - 1)
                .unwrap()
                .as_str(),
            "Test2"
        );

        let mut sql = String::new();
        where_builder.build(&mut sql);

        println!("{}", sql);
    }
}
