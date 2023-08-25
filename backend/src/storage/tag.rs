use std::collections::HashMap;
use std::convert::TryFrom;

use crate::models::{InvalidTag, InvalidityKind, NewTag, Tag};
use serde::{Deserialize, Serialize};
use sqlx::error::ErrorKind;
use sqlx::postgres::{PgConnection, PgDatabaseError};
use sqlx::{query, query_as};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidTag {
    type Error = &'static str;

    fn try_from(value: &DBConstraint) -> Result<Self, &'static str> {
        match value.0.as_str() {
            "tags_uq_name" => Ok(InvalidTag {
                name: Some(InvalidityKind::AlreadyUsed),
            }),
            _ => Err("Unknown DB constraint {value.0}"),
        }
    }
}

pub async fn get_all_tags(db_conn: &mut PgConnection) -> Result<Vec<Tag>, StorageError> {
    let rows: Vec<Tag> = query_as!(
        Tag,
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

pub async fn add_tag(db_conn: &mut PgConnection, new_tag: &NewTag) -> Result<i32, StorageError> {
    let new_id: i32 = query!(
        "
            INSERT INTO tags (name)
                VALUES ($1)
            RETURNING id;
        ",
        new_tag.name
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_tag_by_name(
    db_conn: &mut PgConnection,
    name: &str,
) -> Result<Option<Tag>, StorageError> {
    let row: Option<Tag> = query_as!(
        Tag,
        "
            SELECT
                id,
                name
            FROM tags
            WHERE name = $1
        ",
        name
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn get_tag_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Tag>, StorageError> {
    let row: Option<Tag> = query_as!(
        Tag,
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

pub async fn replace_tag(
    db_conn: &mut PgConnection,
    id: i32,
    new_tag: &NewTag,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            UPDATE tags SET
                name = $1
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

pub async fn delete_tag(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, StorageError> {
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
