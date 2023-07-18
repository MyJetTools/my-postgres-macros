use my_postgres_macros::{
    DbEnumAsString, DbEnumAsStringWithModel, DbEnumAsU16WithModel, InsertDbEntity,
    MyPostgresJsonModel, SelectDbEntity, TableSchema, UpdateDbEntity, WhereDbModel,
};
use rust_extensions::date_time::DateTimeAsMicroseconds;
use serde::{Deserialize, Serialize};

#[derive(SelectDbEntity, InsertDbEntity, UpdateDbEntity, TableSchema)]
pub struct TestJsonField {
    #[primary_key]
    pub id: i32,

    #[sql_type("timestamp")]
    pub date: DateTimeAsMicroseconds,

    #[db_field_name("myTest")]
    #[sql_type("jsonb")]
    pub test: MyStructure,

    pub my_enum_no_model: MyEnum,

    #[db_field_name(model_field_name = "field_model")]
    pub my_enum: MyEnumWithModel,

    pub b: bool,

    pub c: f32,
    pub d: f64,
}

#[derive(Serialize, Deserialize, MyPostgresJsonModel)]
pub struct MyStructure {
    pub a: i32,
    pub b: bool,
}

#[derive(Serialize, Deserialize, DbEnumAsString)]
pub enum MyEnum {
    #[enum_case("Test")]
    Case1,
    #[enum_case("Test2")]
    Case2,
}

#[derive(Serialize, Deserialize, DbEnumAsStringWithModel)]
pub enum MyEnumWithModel {
    #[enum_case("Test")]
    Case1(MyStructure),
    #[enum_case("Test2")]
    Case2(MyStructure),
}

#[derive(Serialize, Deserialize, DbEnumAsU16WithModel)]
pub enum MyEnumWordWithModel {
    #[enum_case(1)]
    Case1(MyStructure),
    #[enum_case(2)]
    Case2(MyStructure),
}

#[derive(WhereDbModel)]
pub struct WhereModel {
    #[limit]
    pub limit: usize,
}

#[cfg(test)]
mod tests {

    use my_postgres::{sql::SelectBuilder, UpdateConflictType};
    use rust_extensions::date_time::DateTimeAsMicroseconds;

    use super::{TestJsonField, WhereModel};

    use my_postgres::sql_select::SelectEntity;

    #[test]
    fn test_fill_select_fields() {
        let mut select_builder = SelectBuilder::new();

        TestJsonField::fill_select_fields(&mut select_builder);

        let mut sql = String::new();

        select_builder.fill_select_fields(&mut sql);

        println!("{}", sql);
    }

    #[test]
    fn test_insert() {
        let date =
            DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap();

        let entity = TestJsonField {
            id: 12,
            date,
            test: super::MyStructure { a: 4, b: true },
            my_enum_no_model: super::MyEnum::Case1,
            my_enum: super::MyEnumWithModel::Case1(super::MyStructure { a: 5, b: false }),
            b: false,
            c: 1.11,
            d: 2.22,
        };

        let sql = my_postgres::sql::build_insert_sql(&entity, "test");

        println!("{}", sql.sql);
    }

    #[test]
    fn test_insert_or_update() {
        let date =
            DateTimeAsMicroseconds::parse_iso_string("2023-06-19T22:07:20.518741+00:00").unwrap();

        let entity = TestJsonField {
            id: 12,
            date,
            test: super::MyStructure { a: 4, b: true },
            my_enum_no_model: super::MyEnum::Case1,
            my_enum: super::MyEnumWithModel::Case1(super::MyStructure { a: 5, b: false }),
            b: false,
            c: 1.11,
            d: 2.22,
        };

        let sql = my_postgres::sql::build_insert_or_update_sql(
            &entity,
            "test",
            &UpdateConflictType::OnPrimaryKeyConstraint("pk_name".into()),
        );

        println!("{}", sql.sql);
    }

    #[test]
    fn test_select() {
        let where_model = WhereModel { limit: 10 };

        let select_builder = SelectBuilder::from_select_model::<TestJsonField>();
        let sql = select_builder.build_select_sql("test", Some(&where_model));

        println!("{}", sql.sql);
    }
}
