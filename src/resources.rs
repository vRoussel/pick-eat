pub mod category;
pub mod ingredient;
pub mod recipe;
pub mod tag;
pub mod unit;

pub async fn get_total_count(
    db_conn: &tokio_postgres::Client,
    table_name: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let query = format!("SELECT count(*) FROM {}", table_name);
    db_conn
        .query(query.as_str(), &[])
        .await
        .map(|rows| rows[0].get(0))
        .map_err(|e| e.into())
}
