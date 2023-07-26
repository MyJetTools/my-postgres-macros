#[cfg(test)]
mod tests {
    use my_postgres::table_schema::TableSchemaProvider;
    use my_postgres_macros::TableSchema;

    //#[derive(TableSchema)]
    pub struct AccountDbModel {
        //  #[primary_key]
        pub id: String,
        // #[primary_key]
        pub currency: String,
        pub field_3: String,
    }

    impl my_postgres::table_schema::TableSchemaProvider for AccountDbModel {
        const PRIMARY_KEY_COLUMNS: Option<&'static [&'static str]> = Some(&[
            my_postgres::ColumnName::new("id".into()),
            my_postgres::ColumnName::new("currency".into()),
        ]);
        fn get_columns() -> Vec<my_postgres::table_schema::TableColumn> {
            use my_postgres::table_schema::*;
            vec![
                TableColumn {
                    name: "id".into(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
                TableColumn {
                    name: "currency".into(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
                TableColumn {
                    name: "field_3".into(),
                    sql_type: String::get_sql_type(None),
                    is_nullable: false,
                    default: None,
                },
            ]
        }
        fn get_indexes(
        ) -> Option<std::collections::HashMap<String, my_postgres::table_schema::IndexSchema>>
        {
            use my_postgres::table_schema::*;
            None
        }
    }

    #[test]
    fn test_partition_key() {
        let primary_keys = AccountDbModel::PRIMARY_KEY_COLUMNS.unwrap();

        assert_eq!(primary_keys.len(), 2);
        assert_eq!(primary_keys.get(0).unwrap().name.as_str(), "id");
        assert_eq!(primary_keys.get(1).unwrap().name.as_str(), "currency");
    }
}
