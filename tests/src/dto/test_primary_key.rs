#[cfg(test)]
mod tests {
    use my_postgres::table_schema::TableSchemaProvider;
    use my_postgres_macros::TableSchema;

    #[derive(TableSchema)]
    pub struct AccountDbModel {
        #[primary_key(0)]
        pub id: String,
        pub currency: String,
    }
    fn test_partition_key() {
        assert_eq!(AccountDbModel::PRIMARY_KEY_COLUMNS[0], "id");
        assert_eq!(AccountDbModel::PRIMARY_KEY_COLUMNS[0], "id");
    }
}
