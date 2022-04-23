pub const QUERY_SINGLE_ROW: &str = r###"pub async fn query_single_row(
    client: &tokio_postgres::Client,
    select: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Option<Self>, tokio_postgres::Error> {
    let rows = client.query(select, params).await?;

    if let Some(row) = rows.get(0) {
        Ok(Some(Self::from_db_row(row)))
    } else {
        Ok(None)
    }
}"###;

pub const QUERY_ROWS: &str = r###"pub async fn query_rows(
    client: &tokio_postgres::Client,
    select: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<Vec<Self>, tokio_postgres::Error> {
    let result = client
        .query(select, params)
        .await?
        .iter()
        .map(|itm| Self::from_db_row(itm))
        .collect();

    Ok(result)
}
"###;
