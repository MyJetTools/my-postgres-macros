#[cfg(test)]
mod tests {

    use my_postgres::table_schema::IndexOrder;
    use my_postgres_macros::TableSchema;
    #[derive(TableSchema)]
    pub struct TableSchemaModel {
        #[primary_key(0)]
        pub primary_key_first: String,
        #[primary_key(1)]
        #[db_index(id:0, index_name: "test_index", is_unique: true, order: "ASC")]
        pub primary_key_second: String,
        #[db_index(id:1, index_name: "test_index", is_unique: true, order: "DESC")]
        #[default_value("default_value")]
        pub string_column: String,
        pub int_column: i32,

        #[ignore_table_column]
        pub to_be_ignored: String,
    }

    impl TableSchemaModel {
        fn get_columns() -> Vec<my_postgres::table_schema::TableColumn> {
            use my_postgres::table_schema::*;
            vec![
                TableColumn {
                    name: "primary_key_first".to_string(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
                TableColumn {
                    name: "primary_key_second".to_string(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
                TableColumn {
                    name: "string_column".to_string(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: Some("default_value"),
                },
                TableColumn {
                    name: "int_column".to_string(),
                    sql_type: i32::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
            ]
        }
        fn get_indexes(
        ) -> Option<std::collections::HashMap<String, my_postgres::table_schema::IndexSchema>>
        {
            use my_postgres::table_schema::*;
            let mut result = std::collections::HashMap::new();
            result.insert(
                "test_index".to_string(),
                IndexSchema::new(
                    true,
                    vec![
                        IndexField {
                            name: "primary_key_second".into(),
                            order: IndexOrder::Asc,
                        },
                        IndexField {
                            name: "string_column".into(),
                            order: IndexOrder::Desc,
                        },
                    ],
                ),
            );
            Some(result)
        }
    }

    #[test]
    fn tests_primary_key_generation() {
        use my_postgres::table_schema::TableSchemaProvider;

        let columns = TableSchemaModel::get_columns();

        assert_eq!(columns.len(), 4);

        let primary_key_columns = TableSchemaModel::PRIMARY_KEY_COLUMNS.unwrap();
        assert_eq!(primary_key_columns.len(), 2);

        assert_eq!(primary_key_columns[0], "primary_key_first");
        assert_eq!(primary_key_columns[1], "primary_key_second");

        let indexes = TableSchemaModel::get_indexes().unwrap();
        assert_eq!(indexes.len(), 1);

        let test_index = indexes.get("test_index").unwrap();

        assert!(test_index.is_unique);

        assert_eq!(test_index.fields[0].name.as_str(), "primary_key_second");
        assert!(test_index.fields[0].order.is_the_same_to(&IndexOrder::Asc));

        assert_eq!(test_index.fields[1].name.as_str(), "string_column");
        assert!(test_index.fields[1].order.is_the_same_to(&IndexOrder::Desc));
    }

    #[derive(TableSchema)]
    pub struct TableSchemaWithRenamedColumnModel {
        #[primary_key(0)]
        pub primary_key_first: String,
        #[primary_key(1)]
        #[db_field_name("the_second_column")]
        #[db_index(id:0, index_name: "test_index", is_unique: true, order: "ASC")]
        pub primary_key_second: String,
        #[db_index(id:1, index_name: "test_index", is_unique: true, order: "ASC")]
        pub string_column: String,
        pub int_column: i32,
    }

    #[test]
    fn tests_primary_key_generation_with_renamed_column() {
        use my_postgres::table_schema::TableSchemaProvider;

        let primary_key_columns = TableSchemaWithRenamedColumnModel::PRIMARY_KEY_COLUMNS.unwrap();
        assert_eq!(primary_key_columns.len(), 2);

        assert_eq!(primary_key_columns[0], "primary_key_first");
        assert_eq!(primary_key_columns[1], "the_second_column");
    }
}
