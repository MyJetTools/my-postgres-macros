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

#[cfg(test)]
mod tests {

    use my_postgres::{sql_insert::SqlInsertModel, UpdateConflictType};

    use super::*;

    #[test]
    fn test_e_tag_is_generated_as_insert_model() {
        use my_postgres::sql_insert::SqlInsertModel;
        assert_eq!(TestETagDto::get_e_tag_column_name().unwrap(), "etag");
    }

    #[test]
    fn test_e_tag_is_generated_as_update_model() {
        assert_eq!(TestETagDto::get_e_tag_column_name().unwrap(), "etag");
    }

    #[test]
    fn test_upsert_sql() {
        let date = DateTimeAsMicroseconds::now();

        let entity = TestETagDto {
            id: 4,
            date,
            e_tag: 2,
        };

        let sql = my_postgres::sql::build_upsert_sql(
            &entity,
            "test_table_name",
            &UpdateConflictType::OnPrimaryKeyConstraint("pk_name".into()),
            6,
        );

        println!("{}", sql.sql);
    }
}
