use my_postgres_macros::{
    DbEnumAsStringWithModel, InsertDbEntity, MyPostgresJsonModel, SelectDbEntity, UpdateDbEntity,
};
use serde::{Deserialize, Serialize};

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity)]
pub struct KeyValue {
    #[primary_key]
    pub client_id: String,
    #[primary_key]
    pub key: String,
    #[db_field_name(model_field_name = "field_model")]
    pub value: MyEnumWithModel,
}

#[derive(DbEnumAsStringWithModel)]
pub enum MyEnumWithModel {
    #[enum_case("Test")]
    Case1(MyStructure),
    #[enum_case("Test2")]
    Case2(MyStructure2),
}

#[derive(Serialize, Deserialize, MyPostgresJsonModel)]
pub struct MyStructure {
    pub a: i32,
    pub b: bool,
}
#[derive(Serialize, Deserialize, MyPostgresJsonModel)]
pub struct MyStructure2 {
    pub c: i32,
    pub d: bool,
}

#[cfg(test)]
mod tests {

    use my_postgres::{sql::SelectBuilder, sql_select::SelectEntity, UpdateConflictType};
    use my_postgres_macros::WhereDbModel;

    use super::*;

    #[derive(WhereDbModel)]
    pub struct TestWhereModel {
        a: i64,
    }

    #[test]
    fn test_insert_case() {
        let model = KeyValue {
            client_id: "client1".to_string(),
            key: "key1".to_string(),
            value: MyEnumWithModel::Case1(MyStructure { a: 1, b: true }),
        };

        let sql = my_postgres::sql::build_insert_sql(&model, "test_table_name");

        assert_eq!(
            sql.sql,
            "INSERT INTO test_table_name(client_id,key,value,field_model) VALUES ($1,$2,$3,$4)"
        );

        assert_eq!(sql.values.len(), 4);
        assert_eq!(sql.values.get(0).unwrap().as_str(), "client1");
        assert_eq!(sql.values.get(1).unwrap().as_str(), "key1");
        assert_eq!(sql.values.get(2).unwrap().as_str(), "Test");
        assert_eq!(sql.values.get(3).unwrap().as_str(), "{\"a\":1,\"b\":true}");
    }

    #[test]
    fn test_bulk_insert_or_update() {
        let models = vec![
            KeyValue {
                client_id: "client1".to_string(),
                key: "key1".to_string(),
                value: MyEnumWithModel::Case1(MyStructure { a: 1, b: true }),
            },
            KeyValue {
                client_id: "client1".to_string(),
                key: "key2".to_string(),
                value: MyEnumWithModel::Case2(MyStructure2 { c: 1, d: true }),
            },
        ];

        let sql = my_postgres::sql::build_bulk_insert_or_update_sql(
            "test",
            &UpdateConflictType::OnPrimaryKeyConstraint("pk_name".into()),
            &models,
        );

        assert_eq!(sql.sql, "INSERT INTO test(client_id,key,value,field_model) VALUES ($1,$2,$3,$4),($1,$5,$6,$7) ON CONFLICT ON CONSTRAINT pk_name DO UPDATE SET value=EXCLUDED.value,field_model=EXCLUDED.field_model");

        assert_eq!(sql.values.get(0).unwrap().as_str(), "client1");
        assert_eq!(sql.values.get(1).unwrap().as_str(), "key1");
        assert_eq!(sql.values.get(2).unwrap().as_str(), "Test");
        assert_eq!(sql.values.get(3).unwrap().as_str(), "{\"a\":1,\"b\":true}");

        assert_eq!(sql.values.get(4).unwrap().as_str(), "key2");
        assert_eq!(sql.values.get(5).unwrap().as_str(), "Test2");
        assert_eq!(sql.values.get(6).unwrap().as_str(), "{\"c\":1,\"d\":true}");
    }

    #[test]
    fn test_select_fields() {
        let mut builder = SelectBuilder::new();
        KeyValue::fill_select_fields(&mut builder);

        assert_eq!(builder.len(), 4);

        assert_eq!(builder.get(0).unwrap().unwrap_as_field(), "client_id");
        assert_eq!(builder.get(1).unwrap().unwrap_as_field(), "key");
        assert_eq!(builder.get(2).unwrap().unwrap_as_field(), "value");
        assert_eq!(builder.get(3).unwrap().unwrap_as_field(), "field_model");
    }

    #[test]
    fn test_select_sql_with_no_where() {
        let select_builder = SelectBuilder::from_select_model::<KeyValue>();

        let sql = select_builder.build_select_sql::<TestWhereModel>("table_name", None);

        assert_eq!(
            "SELECT client_id,key,value,field_model FROM table_name",
            sql.sql
        );
        assert_eq!(sql.values.len(), 0);
    }

    #[test]
    fn test_select_sql_with_basic_where_case() {
        let where_model = TestWhereModel { a: 6 };

        let select_builder = SelectBuilder::from_select_model::<KeyValue>();

        let sql = select_builder.build_select_sql("table_name", Some(&where_model));

        assert_eq!(
            "SELECT client_id,key,value,field_model FROM table_name WHERE a=6",
            sql.sql
        );
        assert_eq!(sql.values.len(), 0);
    }

    #[test]
    fn test_update_case() {
        let entity = KeyValue {
            client_id: "client1".to_string(),
            key: "key1".to_string(),
            value: MyEnumWithModel::Case1(MyStructure { a: 1, b: true }),
        };

        let sql = my_postgres::sql::build_update_sql(&entity, "test");

        assert_eq!(
            sql.sql,
            "UPDATE test SET (value,field_model)=($1,$2) WHERE client_id=$3 AND key=$4"
        );

        assert_eq!(sql.values.len(), 4);

        assert_eq!(sql.values.get(0).unwrap().as_str(), "Test");
        assert_eq!(sql.values.get(1).unwrap().as_str(), "{\"a\":1,\"b\":true}");
        assert_eq!(sql.values.get(2).unwrap().as_str(), "client1");
        assert_eq!(sql.values.get(3).unwrap().as_str(), "key1");
    }
}