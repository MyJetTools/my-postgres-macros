use my_postgres_macros::{
    DbEnumAsString, InsertDbEntity, SelectDbEntity, UpdateDbEntity, WhereDbModel,
};

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity)]
pub struct KeyValue {
    #[primary_key]
    pub client_id: MyEnumAsString,
    #[primary_key]
    pub key: String,
    pub value: String,
}

#[derive(DbEnumAsString)]
pub enum MyEnumAsString {
    #[enum_case("TestCase1")]
    Case1,
    #[enum_case("TestCase2")]
    Case2,
}

#[cfg(test)]
mod tests {

    use my_postgres::{
        sql::SelectBuilder, sql_insert::SqlInsertModel, sql_select::SelectEntity,
        sql_update::SqlUpdateModel, UpdateConflictType,
    };

    use super::*;

    #[derive(WhereDbModel)]
    pub struct TestWhereModel {
        a: i64,
    }

    #[test]
    fn test_insert_case() {
        let models = KeyValue {
            client_id: MyEnumAsString::Case1,
            key: "key1".to_string(),
            value: "value1".to_string(),
        };

        let (sql, params) = models.build_insert_sql("test_table_name");

        assert_eq!(
            sql,
            "INSERT INTO test_table_name(client_id,key,value) VALUES ($1,$2,$3)"
        );

        assert_eq!(params.len(), 3);
        assert_eq!(params.get(0).unwrap().as_str(), "TestCase1");
        assert_eq!(params.get(1).unwrap().as_str(), "key1");
        assert_eq!(params.get(2).unwrap().as_str(), "value1");
    }

    #[test]
    fn test_bulk_insert_or_update() {
        let models = vec![
            KeyValue {
                client_id: MyEnumAsString::Case1,
                key: "key1".to_string(),
                value: "value1".to_string(),
            },
            KeyValue {
                client_id: MyEnumAsString::Case2,
                key: "key2".to_string(),
                value: "value2".to_string(),
            },
        ];

        let (sql, params) = KeyValue::build_bulk_insert_or_update_sql(
            "test",
            &UpdateConflictType::OnPrimaryKeyConstraint("pk_name".into()),
            &models,
        );

        assert_eq!(
            sql,
            "INSERT INTO test(client_id,key,value) VALUES ($1,$2,$3),($4,$5,$6) ON CONFLICT ON CONSTRAINT pk_name DO UPDATE SET value=EXCLUDED.value"
        );

        assert_eq!(params.len(), 6);

        assert_eq!(params.get(0).unwrap().as_str(), "TestCase1");
        assert_eq!(params.get(1).unwrap().as_str(), "key1");
        assert_eq!(params.get(2).unwrap().as_str(), "value1");

        assert_eq!(params.get(3).unwrap().as_str(), "TestCase2");
        assert_eq!(params.get(4).unwrap().as_str(), "key2");
        assert_eq!(params.get(5).unwrap().as_str(), "value2");
    }

    #[test]
    fn test_select_sql_with_no_where() {
        let select_builder = SelectBuilder::from_select_model::<KeyValue>();
        let (sql, params) = select_builder.build_select_sql::<TestWhereModel>("table_name", None);

        assert_eq!("SELECT client_id,key,value FROM table_name", sql);
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_select_fields() {
        let mut builder = SelectBuilder::new();
        KeyValue::fill_select_fields(&mut builder);

        assert_eq!(builder.len(), 3);

        assert_eq!(builder.get(0).unwrap().unwrap_as_field(), "client_id");
        assert_eq!(builder.get(1).unwrap().unwrap_as_field(), "key");
        assert_eq!(builder.get(2).unwrap().unwrap_as_field(), "value");
    }

    #[test]
    fn test_select_sql_with_basic_where_case() {
        let where_model = TestWhereModel { a: 6 };

        let select_builder = SelectBuilder::from_select_model::<KeyValue>();
        let (sql, params) = select_builder.build_select_sql("table_name", Some(&where_model));

        assert_eq!("SELECT client_id,key,value FROM table_name WHERE a=6", sql);
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_update_case() {
        let entity = KeyValue {
            client_id: MyEnumAsString::Case1,
            key: "key1".to_string(),
            value: "value1".to_string(),
        };

        let (sql, params) = entity.build_update_sql("test", Some(&entity));

        assert_eq!(
            sql,
            "UPDATE test SET value=$1 WHERE client_id=$2 AND key=$3"
        );

        assert_eq!(params.len(), 3);

        assert_eq!(params.get(0).unwrap().as_str(), "value1");
        assert_eq!(params.get(1).unwrap().as_str(), "TestCase1");
        assert_eq!(params.get(2).unwrap().as_str(), "key1");
    }
}
