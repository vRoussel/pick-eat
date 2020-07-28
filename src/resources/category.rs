use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, error::Error};

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

pub async fn get_many(db_conn: &Client, min: i64, max: i64) -> Result<Vec<FromDB>, Error> {
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
}

pub async fn add_one(db_conn: &Client, new_category: &New) -> Result<i32, Error> {
    let insert_query = "\
        INSERT INTO categories (name) \
            VALUES ($1) \
        RETURNING id;
    ";
    db_conn.query(insert_query, &[&new_category.name])
        .await
        .map(|rows| rows[0].get(0))
}
