use std::convert::TryFrom;

use crate::models::{Diet, InvalidDiet, InvalidityKind, NewDiet};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidDiet {
    type Error = &'static str;

    fn try_from(value: &DBConstraint) -> Result<Self, &'static str> {
        match value.0.as_str() {
            "diets_uq_name" => Ok(InvalidDiet {
                name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            "diets_uq_label" => Ok(InvalidDiet {
                label: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            _ => Err("Unknown DB constraint {value.0}"),
        }
    }
}

pub async fn get_all_diets(db_conn: &mut PgConnection) -> Result<Vec<Diet>, StorageError> {
    let rows: Vec<Diet> = query_as!(
        Diet,
        "
            SELECT
                id,
                name,
                label
            FROM diets
            ORDER BY name
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_diet(db_conn: &mut PgConnection, new_diet: &NewDiet) -> Result<i32, StorageError> {
    let new_id: i32 = query!(
        "
            INSERT INTO diets (name, label)
                VALUES ($1, $2)
            RETURNING id;
        ",
        new_diet.name,
        new_diet.label
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_diet_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Diet>, StorageError> {
    let row: Option<Diet> = query_as!(
        Diet,
        "
            SELECT
                id,
                name,
                label
            FROM diets
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn replace_diet(
    db_conn: &mut PgConnection,
    id: i32,
    new_diet: &NewDiet,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            UPDATE diets SET
                name = $1,
                label = $2
            WHERE id = $3
        ",
        new_diet.name,
        new_diet.label,
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

pub async fn delete_diet(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            DELETE FROM diets
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
