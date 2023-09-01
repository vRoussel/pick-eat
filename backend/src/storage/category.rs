use std::convert::TryFrom;

use crate::models::{Category, InvalidCategory, InvalidityKind, NewCategory};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidCategory {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "categories_uq_name" => Ok(InvalidCategory {
                name: Some(InvalidityKind::AlreadyUsed),
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn get_all_categories(db_conn: &mut PgConnection) -> Result<Vec<Category>, StorageError> {
    let rows: Vec<Category> = query_as!(
        Category,
        "
            SELECT
                id,
                name
            FROM categories
            ORDER BY name
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_category(
    db_conn: &mut PgConnection,
    new_category: &NewCategory,
) -> Result<i32, StorageError> {
    let new_id: i32 = query!(
        "
            INSERT INTO categories (name)
                VALUES ($1)
            RETURNING id;
        ",
        new_category.name
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_category_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Category>, StorageError> {
    let row: Option<Category> = query_as!(
        Category,
        "
            SELECT
                id,
                name
            FROM categories
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn replace_category(
    db_conn: &mut PgConnection,
    id: i32,
    new_category: &NewCategory,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            UPDATE categories SET
                name = $1
            WHERE id = $2
        ",
        new_category.name,
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

pub async fn delete_category(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            DELETE FROM categories
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
