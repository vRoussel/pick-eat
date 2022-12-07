use log::debug;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as, Error};

use super::unit;

#[derive(Debug, Serialize)]
pub struct FromDB {
    id: i32,
    name: String,
    default_unit: Option<unit::FromDB>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    name: String,
    default_unit_id: Option<i32>,
}

pub mod quantified {
    use super::unit;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Full {
        id: i32,
        name: String,
        quantity: Option<f32>,
        unit: Option<unit::FromDB>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ref {
        pub(crate) id: i32,
        pub(crate) quantity: Option<f32>,
        pub(crate) unit_id: Option<i32>,
    }
}

pub async fn get_all(db_conn: &mut PgConnection) -> Result<Vec<FromDB>, Error> {
    let rows: Vec<FromDB> = query_as!(
        FromDB,
        r#"
            SELECT
                i.id as id,
                i.name as name,
                CASE WHEN i.default_unit_id is null THEN
                    null
                ELSE
                    (
                    u.id,
                    u.full_name,
                    u.short_name
                    )
                END as "default_unit: unit::FromDB"
            FROM
                ingredients as i
                LEFT JOIN units as u
                ON i.default_unit_id = u.id
            ORDER BY name
        "#,
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_one(db_conn: &mut PgConnection, new_ingredient: &New) -> Result<i32, Error> {
    debug!("hello1");
    let new_id: i32 = query!(
        "
            INSERT INTO ingredients (name, default_unit_id)
                VALUES (sentence_case($1), $2)
            RETURNING id;
        ",
        new_ingredient.name,
        new_ingredient.default_unit_id
    )
    .fetch_one(db_conn)
    .await?
    .id;

    debug!("hello2");
    Ok(new_id)
}

pub async fn get_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        r#"
            SELECT
                i.id as id,
                i.name as name,
                CASE WHEN i.default_unit_id is null THEN
                    null
                ELSE
                    (
                    u.id,
                    u.full_name,
                    u.short_name
                    )
                END as "default_unit: unit::FromDB"
            FROM
                ingredients as i
                LEFT JOIN units as u
                ON i.default_unit_id = u.id
            WHERE
                i.id = $1
        "#,
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn modify_one(
    db_conn: &mut PgConnection,
    id: i32,
    new_ingredient: &New,
) -> Result<Option<()>, Error> {
    let n_rows: u64 = query!(
        "
            UPDATE ingredients SET
                name = sentence_case($1),
                default_unit_id = $2
            WHERE id = $3
        ",
        new_ingredient.name,
        new_ingredient.default_unit_id,
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
            DELETE FROM ingredients
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
