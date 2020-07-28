use serde::{Deserialize, Serialize};
use tokio_postgres::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String
}

impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            name: row.get("name")
        }
    }
}

pub async fn get_many(db_conn: &Client, min: i64, max: i64) -> Result<Vec<FromDB>, Box<dyn std::error::Error>> {
    let categories_query = "\
        SELECT \
            id, \
            name \
        FROM categories \
        ORDER BY name \
        OFFSET $1 \
        LIMIT $2;
    ";

    db_conn.query(categories_query, &[&(min-1), &(max-min+1)])
        .await
        .map(|rows|rows.iter().map(|r| r.into()).collect())
        .map_err(|e|e.into())
}
