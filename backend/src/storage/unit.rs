use std::convert::TryFrom;

use crate::models::{InvalidUnit, InvalidityKind, NewUnit, Unit};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidUnit {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "units_uq_full_name" => Ok(InvalidUnit {
                full_name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            "units_uq_short_name" => Ok(InvalidUnit {
                short_name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn get_all_units(db_conn: &mut PgConnection) -> Result<Vec<Unit>, StorageError> {
    let rows: Vec<Unit> = query_as!(
        Unit,
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

pub async fn add_unit(db_conn: &mut PgConnection, new_unit: &NewUnit) -> Result<i32, StorageError> {
    let new_id: i32 = query!(
        "
            INSERT INTO units (full_name, short_name)
                VALUES ($1, $2)
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

pub async fn get_unit_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Unit>, StorageError> {
    let row: Option<Unit> = query_as!(
        Unit,
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

pub async fn replace_unit(
    db_conn: &mut PgConnection,
    id: i32,
    new_unit: &NewUnit,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            UPDATE units SET
                full_name = $1,
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

pub async fn delete_unit(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, StorageError> {
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
