use std::convert::TryFrom;

use crate::models::{Ingredient, InvalidIngredient, InvalidityKind, NewIngredient, Unit};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidIngredient {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "ingredients_uq_name" => Ok(InvalidIngredient {
                name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            "ingredients_fk_default_unit" => Ok(InvalidIngredient {
                default_unit_id: Some(InvalidityKind::InvalidRef),
                ..Default::default()
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn get_all_ingredients(
    db_conn: &mut PgConnection,
) -> Result<Vec<Ingredient>, StorageError> {
    let rows: Vec<Ingredient> = query_as!(
        Ingredient,
        r#"
            SELECT
                id as "id!",
                name as "name!",
                default_unit as "default_unit: Unit"
            FROM ingredients_full
            ORDER BY name
        "#,
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_ingredient(
    db_conn: &mut PgConnection,
    new_ingredient: &NewIngredient,
) -> Result<i32, StorageError> {
    let new_id: i32 = query!(
        "
            INSERT INTO ingredients (name, default_unit_id)
                VALUES ($1, $2)
            RETURNING id;
        ",
        new_ingredient.name,
        new_ingredient.default_unit_id
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn get_ingredient_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Ingredient>, StorageError> {
    let row: Option<Ingredient> = query_as!(
        Ingredient,
        r#"
            SELECT
                id as "id!",
                name as "name!",
                default_unit as "default_unit!: Unit"
            FROM ingredients_full
            WHERE id = $1
        "#,
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn replace_ingredient(
    db_conn: &mut PgConnection,
    id: i32,
    new_ingredient: &NewIngredient,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            UPDATE ingredients SET
                name = $1,
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

pub async fn delete_ingredient(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<()>, StorageError> {
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
