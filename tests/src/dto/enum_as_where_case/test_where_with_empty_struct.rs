use my_postgres_macros::*;

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity)]
struct KeyValue {
    #[primary_key]
    pub client_id: String,
    #[primary_key]
    pub key: String,
    pub value: String,
}

#[derive(WhereDbModel)]
struct WhereAllModel {}

#[derive(DbEnumAsI32, Copy, Clone)]
pub enum TradingPlatformDto {
    #[enum_case(value: 0)]
    MetaTrader4,
    #[enum_case(value: 1)]
    MetaTrader5,
}

#[cfg(test)]
mod test {
    use my_postgres::sql::SelectBuilder;

    use super::{KeyValue, WhereAllModel};

    #[test]
    fn test_with_empty_where() {
        let sql = SelectBuilder::from_select_model::<KeyValue>();

        let sql = sql.build_select_sql::<WhereAllModel>("test", None);

        println!("{}", sql.sql);
    }
}
