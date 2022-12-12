pub mod category;
pub mod ingredient;
pub mod qingredient;
pub mod recipe;
pub mod season;
pub mod tag;
pub mod unit;

use sqlx::{postgres::PgConnection, query, Row};

pub async fn get_total_count(
    db_conn: &mut PgConnection,
    table_name: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let q = format!("SELECT count(*) from {}", table_name);
    let count: i64 = query(&q).fetch_one(db_conn).await?.get(0);

    Ok(count)
}

pub async fn isolate_transaction(
    db_conn: &mut PgConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;")
        .execute(db_conn)
        .await?;
    Ok(())
}
