use my_postgres::{sql::SqlValues, sql_where::SqlWhereModel};
use my_postgres_macros::TableSchema;
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(TableSchema)]
pub struct MyTableModel {
    #[generate_where_model(name:"ByTraderIdAndDateWhereModel")]
    pub trader_id: String,

    #[sql_type("timestamp")]
    #[generate_where_model(name:"ByTraderIdAndDateWhereModel", operator: ">")]
    pub date: DateTimeAsMicroseconds,
}

#[test]
fn test_where_auto_generator_with_operator() {
    let where_model = ByTraderIdAndDateWhereModel {
        trader_id: "test".to_string(),
        date: DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap(),
    };

    let mut params = SqlValues::new();
    let where_builder: my_postgres::sql::WhereBuilder =
        where_model.build_where_sql_part(&mut params);

    let result = where_builder.get(0).unwrap();
    assert_eq!(result.db_column_name.name.as_str(), "trader_id");
    assert_eq!(result.op, "=");

    let result = where_builder.get(1).unwrap();
    assert_eq!(result.db_column_name.name.as_str(), "date");
    assert_eq!(result.op, ">");
}
