use crate::query_params::Range;
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tokio_postgres::{error::Error, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
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

pub async fn get_many(db_conn: &Client, range: &Option<Range>) -> Result<Vec<FromDB>, Error> {
    let mut tags_query = String::from(
        "\
        SELECT \
            id, \
            name \
        FROM tags \
        ORDER BY name \
    ",
    );

    let mut params: Vec<Box<dyn ToSql + Sync>> = Vec::new();
    if let Some(r) = range {
        tags_query.push_str(
            " \
            OFFSET $1 \
            LIMIT $2 \
        ",
        );
        params.push(Box::new(r.from - 1));
        params.push(Box::new(r.to - (r.from - 1)));
    }

    db_conn
        .query(
            tags_query.as_str(),
            params
                .iter()
                .map(|b| b.as_ref())
                .collect::<Vec<&(dyn ToSql + Sync)>>()
                .as_slice(),
        )
        .await
        .map(|rows| rows.iter().map(|r| r.into()).collect())
}

pub async fn add_one(db_conn: &Client, new_tag: &New) -> Result<i32, Error> {
    let insert_query = "\
        INSERT INTO tags (name) \
            VALUES ($1) \
        RETURNING id;
    ";
    db_conn
        .query(insert_query, &[&new_tag.name])
        .await
        .map(|rows| rows[0].get(0))
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let query = "\
        SELECT \
            id, \
            name \
        FROM tags \
        WHERE id = $1 \
    ";

    db_conn
        .query_opt(query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))
}

pub async fn modify_one(db_conn: &Client, id: i32, new_tag: &New) -> Result<Option<()>, Error> {
    let update_query = "\
        UPDATE tags SET \
            name = $1 \
        WHERE id = $2 \
        RETURNING id;
    ";
    db_conn
        .query_opt(update_query, &[&new_tag.name, &id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}

pub async fn delete_one(db_conn: &Client, id: i32) -> Result<Option<()>, Error> {
    let delete_query = "\
        DELETE FROM tags \
        WHERE id = $1 \
        RETURNING id;
    ";
    db_conn
        .query_opt(delete_query, &[&id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}
