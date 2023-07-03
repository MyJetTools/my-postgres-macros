use my_postgres_macros::WhereDbModel;

#[derive(WhereDbModel)]
pub struct WhereByIdModel<'s> {
    pub id: &'s str,
}

#[cfg(test)]
mod test {
    use super::WhereByIdModel;

    #[test]
    fn test() {
        let a = WhereByIdModel { id: "test" };
    }
}
