use my_postgres_macros::WhereDbModel;

#[derive(WhereDbModel)]
pub struct WhereByIdModel<'s> {
    pub id: &'s str,
}

#[cfg(test)]
mod test {
    use my_postgres::{sql::SqlValues, sql_where::SqlWhereModel};

    use super::WhereByIdModel;

    #[test]
    fn test() {
        let where_model = WhereByIdModel { id: "test" };

        let mut params = SqlValues::new();
        let where_builder: my_postgres::sql::WhereBuilder =
            where_model.build_where_sql_part(&mut params);

        let mut sql = String::new();

        where_builder.build(&mut sql);

        assert_eq!("id=$1", sql);

        assert_eq!(params.get(0).unwrap().as_str(), "test")
    }
}
