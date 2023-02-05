use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnection;
use sqlx::{query_as, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    id: i32,
    name: String,
    label: Option<String>,
}

pub type Ref = i32;

pub async fn get_all(db_conn: &mut PgConnection) -> Result<Vec<FromDB>, Error> {
    let rows: Vec<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                name,
                label
            FROM seasons
            ORDER BY id
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn get_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                name,
                label
            FROM seasons
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}
