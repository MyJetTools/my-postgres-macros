#[cfg(test)]
mod tests {
    use my_postgres::sql_select::BulkSelectBuilder;
    use my_postgres_macros::{BulkSelectDbEntity, SelectDbEntity, WhereDbModel};

    #[derive(BulkSelectDbEntity, SelectDbEntity)]
    pub struct BulkKeyValueDto {
        pub line_no: i32,
        pub client_id: String,
        pub key: String,
        pub value: String,
    }

    #[derive(WhereDbModel)]
    pub struct WhereDbModel {
        pub client_id: String,
        pub key: String,
    }

    #[test]
    fn tests() {
        let mut keys: Vec<WhereDbModel> = Vec::new();

        keys.push(WhereDbModel {
            client_id: "test".to_string(),
            key: "test".to_string(),
        });

        let builder = BulkSelectBuilder::new("test", keys);

        let sql = builder.build_sql::<BulkKeyValueDto>();

        assert_eq!("SELECT 0::int as \"line_no\",client_id,key,value FROM test WHERE client_id=$1 AND key=$1\n", sql.sql);
    }
}
