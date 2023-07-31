use my_postgres_macros::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;

#[derive(Debug, Clone, PartialEq, Eq, DbEnumAsI32)]
pub enum ChartTypeDto {
    #[enum_case(id = 0)]
    Balance = 0,
    #[enum_case(id = 1)]
    Equity = 1,
}

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct AccountChartDto {
    #[primary_key(0)]
    pub trader_account_id: String,
    #[primary_key(1)]
    #[sql_type("timestamp")]
    pub date_time: DateTimeAsMicroseconds,
    #[primary_key(2)]
    pub chart_type: ChartTypeDto,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
}

#[derive(WhereDbModel)]
pub struct WhereByAllWithPaginationModel {
    #[limit]
    pub limit: usize,
    #[offset]
    pub offset: usize,
}

#[cfg(test)]
mod tests {
    use my_postgres::sql::SelectBuilder;

    use super::*;

    #[test]
    fn test_select_with_no_where_fields_and_limit_and_take_fields() {
        let select_builder = SelectBuilder::from_select_model::<AccountChartDto>();

        let where_model = WhereByAllWithPaginationModel {
            limit: 2,
            offset: 3,
        };

        let sql = select_builder.to_sql_string("test", Some(&where_model));

        println!("{}", sql.sql);
    }
}
