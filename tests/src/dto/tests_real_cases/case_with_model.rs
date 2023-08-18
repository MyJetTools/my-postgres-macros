use my_postgres_macros::*;
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::*;

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct TraderAccountUpdateLogDto {
    #[primary_key(0)]
    #[generate_update_model(name:"UpdateBrokerDto", param_type:"where")]
    #[generate_where_model("ByAccountIdWhereModel")]
    pub account_id: String,
    #[generate_where_model("ByAccountIdWhereModel")]
    pub client_id: String,

    #[order_by_desc]
    #[sql_type("timestamp")]
    pub created_date: DateTimeAsMicroseconds,

    #[generate_update_model(name:"UpdateBrokerDto", param_type:"update")]
    pub broker: BrokerDto,
    pub trading_platform: TradingPlatformDto,

    pub event_info: UpdateLogDetailsDto,
}

#[derive(DbEnumAsStringWithModel, Clone, Serialize, Deserialize)]
pub enum UpdateLogDetailsDto {
    #[enum_case("WinInPhase1")]
    WinInPhase1(PhaseDetailsDto),
    #[enum_case("WinInPhase2")]
    WinInPhase2(PhaseDetailsDto),
    #[enum_case("LoseInPhase1")]
    LoseInPhase1(PhaseDetailsDto),
    #[enum_case("LoseInPhase2")]
    LoseInPhase2(PhaseDetailsDto),
    #[enum_case("KycRequiredStatusSet")]
    KycRequiredStatusSet(EmptyPhaseDetailsDto),
    #[enum_case("Phase2AccountGranted")]
    Phase2AccountGranted(EmptyPhaseDetailsDto),
}

#[derive(Serialize, Deserialize, MyPostgresJsonModel, Clone)]
pub struct EmptyPhaseDetailsDto {}

#[derive(Serialize, Deserialize, MyPostgresJsonModel, Clone)]
pub struct PhaseDetailsDto {
    pub current_equity: f64,
    pub current_balance: f64,
    pub current_profit: f64,
    pub days_traded: i32,
}

#[derive(DbEnumAsI32, Copy, Clone)]
pub enum TradingPlatformDto {
    #[enum_case(0)]
    MetaTrader4,
    #[enum_case(1)]
    MetaTrader5,
}

#[derive(DbEnumAsI32, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum BrokerDto {
    #[enum_case(0)]
    Welltrade,
}

#[cfg(test)]
mod test {
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::{TraderAccountUpdateLogDto, UpdateLogDetailsDto};

    #[test]
    fn test() {
        let entity = TraderAccountUpdateLogDto {
            account_id: "acc_id".to_string(),
            client_id: "client_id".to_string(),
            created_date: DateTimeAsMicroseconds::from_str("2021-05-05T12:04:23").unwrap(),
            broker: super::BrokerDto::Welltrade,
            trading_platform: super::TradingPlatformDto::MetaTrader4,
            event_info: UpdateLogDetailsDto::LoseInPhase1(super::PhaseDetailsDto {
                current_equity: 15.45,
                current_balance: 22.33,
                current_profit: 1.44,
                days_traded: 55,
            }),
        };

        let sql_data = my_postgres::sql::build_insert_or_update_sql(
            &entity,
            "TEST",
            &my_postgres::UpdateConflictType::OnPrimaryKeyConstraint("PK_NAME".into()),
        );

        println!("{}", sql_data.sql);

        /*
         INSERT INTO TEST
         (account_id, client_id, created_date,               broker,trading_platform, event_info, field_model) VALUES
         ($1,         $2,        '2021-05-05T12:04:23+00:00', 0,    0,                $3,         $4) ON CONFLICT ON CONSTRAINT PK_NAME DO UPDATE SET
                      client_id=EXCLUDED.client_id,
                                 created_date=EXCLUDED.created_date,
                                                             broker=EXCLUDED.broker,
                                                                    trading_platform=EXCLUDED.trading_platform,
                                                                                     event_info=EXCLUDED.event_info,
                                                                                                field_model=EXCLUDED.field_model
        */
    }
}
