use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, error::Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) full_name: String,
    pub(crate) short_name: String
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) full_name: String,
    pub(crate) short_name: String
}

impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            full_name: row.get("full_name"),
            short_name: row.get("short_name")
        }
    }
}

pub async fn get_many(db_conn: &Client, min: i64, max: i64) -> Result<Vec<FromDB>, Error> {
    let units_query = "\
        SELECT \
            id, \
            full_name, \
            short_name \
        FROM units \
        ORDER BY full_name \
        OFFSET $1 \
        LIMIT $2;
    ";

    db_conn.query(units_query, &[&(min-1), &(max-min+1)])
        .await
        .map(|rows|rows.iter().map(|r| r.into()).collect())
}

pub async fn add_one(db_conn: &Client, new_unit: &New) -> Result<i32, Error> {
    let insert_query = "\
        INSERT INTO units (full_name, short_name) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    db_conn.query(insert_query, &[&new_unit.full_name, &new_unit.short_name])
        .await
        .map(|rows| rows[0].get(0))
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let query = "\
        SELECT \
            id, \
            full_name, \
            short_name \
        FROM units \
        WHERE id = $1 \
    ";

    db_conn.query_opt(query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))
}

pub async fn modify_one(db_conn: &Client, id: i32, new_unit: &New) -> Result<Option<()>, Error> {
    let update_query = "\
        UPDATE units SET \
            full_name = $1, \
            short_name = $2
        WHERE id = $3 \
        RETURNING id;
    ";
    db_conn.query_opt(update_query, &[&new_unit.full_name, &new_unit.short_name, &id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}

pub async fn delete_one(db_conn: &Client, id: i32) -> Result<Option<()>, Error> {
    let delete_query = "\
        DELETE FROM units \
        WHERE id = $1 \
        RETURNING id;
    ";
    db_conn.query_opt(delete_query, &[&id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}
