use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tokio_postgres::{error::Error, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
}

impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}

pub async fn get_many(db_conn: &Client) -> Result<Vec<FromDB>, Error> {
    let seasons_query = String::from(
        "
        SELECT
            id,
            name
        FROM seasons
        ORDER BY id
    ",
    );

    db_conn
        .query(seasons_query.as_str(), &[])
        .await
        .map(|rows| rows.iter().map(|r| r.into()).collect())
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let query = "
        SELECT
            id,
            name
        FROM seasons
        WHERE id = $1
    ";

    db_conn
        .query_opt(query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))
}
