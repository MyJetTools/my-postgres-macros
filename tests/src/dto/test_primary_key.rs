#[cfg(test)]
mod tests {
    use my_postgres::table_schema::TableSchemaProvider;
    use my_postgres_macros::TableSchema;

    #[derive(TableSchema)]
    pub struct AccountDbModel {
        #[primary_key]
        pub id: String,
        #[primary_key]
        pub currency: String,
        pub field_3: String,
    }

    #[test]
    fn test_primary_key() {
        let primary_keys = AccountDbModel::get_primary_key_columns().unwrap();

        assert_eq!(primary_keys.len(), 2);
        assert_eq!(primary_keys.get(0).unwrap().name.as_str(), "id");
        assert_eq!(primary_keys.get(1).unwrap().name.as_str(), "currency");
    }
}
