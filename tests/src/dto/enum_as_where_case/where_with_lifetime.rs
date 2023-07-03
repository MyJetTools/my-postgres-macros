use my_postgres_macros::WhereDbModel;

#[derive(WhereDbModel)]
pub struct WhereByIdModel<'s> {
    pub id: &'s str,
}
