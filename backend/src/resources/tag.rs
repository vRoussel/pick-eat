use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnection;
use sqlx::Error;
use sqlx::{query, query_as};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    name: String,
}

pub async fn get_all(db_conn: &mut PgConnection) -> Result<Vec<FromDB>, Error> {
    let rows: Vec<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                name
            FROM tags
            ORDER BY name
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_one(db_conn: &mut PgConnection, new_tag: &New) -> Result<i32, Error> {
    let new_id: i32 = query!(
        "
            INSERT INTO tags (name)
                VALUES (sentence_case($1))
            RETURNING id;
        ",
        new_tag.name
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                name
            FROM tags
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn modify_one(
    db_conn: &mut PgConnection,
    id: i32,
    new_tag: &New,
) -> Result<Option<()>, Error> {
    let n_rows: u64 = query!(
        "
            UPDATE tags SET
                name = sentence_case($1)
            WHERE id = $2
        ",
        new_tag.name,
        id
    )
    .execute(db_conn)
    .await?
    .rows_affected();

    if n_rows > 0 {
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

pub async fn delete_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, Error> {
    let n_rows: u64 = query!(
        "
            DELETE FROM tags
            WHERE id = $1
        ",
        id
    )
    .execute(db_conn)
    .await?
    .rows_affected();

    if n_rows > 0 {
        Ok(Some(()))
    } else {
        Ok(None)
    }
}
