use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnection;
use sqlx::Error;
use sqlx::{query, query_as};

#[derive(Debug, Clone, Serialize, sqlx::Decode, Deserialize)]
pub struct FromDB {
    id: i32,
    full_name: String,
    short_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub full_name: String,
    pub short_name: String,
}

pub type Ref = i32;

pub async fn get_all(db_conn: &mut PgConnection) -> Result<Vec<FromDB>, Error> {
    let rows: Vec<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                full_name,
                short_name
            FROM units
            ORDER BY full_name
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_one(db_conn: &mut PgConnection, new_unit: &New) -> Result<i32, Error> {
    let new_id: i32 = query!(
        "
            INSERT INTO units (full_name, short_name)
                VALUES (sentence_case($1), $2)
            RETURNING id;
        ",
        new_unit.full_name,
        new_unit.short_name
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<FromDB>, Error> {
    let row = query_as!(
        FromDB,
        "
            SELECT
                id,
                full_name,
                short_name
            FROM units
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn get_one_by_full_name(
    db_conn: &mut PgConnection,
    full_name: &str,
) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                full_name,
                short_name
            FROM units
            WHERE full_name = $1
        ",
        full_name
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn get_one_by_short_name(
    db_conn: &mut PgConnection,
    short_name: &str,
) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                id,
                full_name,
                short_name
            FROM units
            WHERE short_name = $1
        ",
        short_name
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn modify_one(
    db_conn: &mut PgConnection,
    id: i32,
    new_unit: &New,
) -> Result<Option<()>, Error> {
    let n_rows: u64 = query!(
        "
            UPDATE units SET
                full_name = sentence_case($1),
                short_name = $2
            WHERE id = $3
        ",
        new_unit.full_name,
        new_unit.short_name,
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
            DELETE FROM units
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
