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
    }

    #[test]
    fn test_partition_key() {
        let primary_keys = AccountDbModel::PRIMARY_KEY_COLUMNS.unwrap();

        assert_eq!(primary_keys.len(), 1);
        assert_eq!(primary_keys.get(0).unwrap().name.as_str(), "id");
    }
}
