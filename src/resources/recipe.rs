use super::ingredient::quantified as QIngredient;
use super::{category, season, tag};
use crate::query_params::Range;
use crate::utils::*;
use log::*;
use serde::{Deserialize, Serialize};
use tokio_postgres::{error::Error, types, types::ToSql, Client};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) notes: String,
    pub(crate) q_ingredients: Vec<QIngredient::Full>,
    pub(crate) categories: Vec<category::FromDB>,
    pub(crate) tags: Vec<tag::FromDB>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: String,
    pub(crate) publish_date: time::Date,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16,
    pub(crate) is_favorite: bool,
    pub(crate) seasons: Vec<season::FromDB>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDBLight {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) image: String,
    pub(crate) is_favorite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String,
    pub(crate) notes: String,
    pub(crate) q_ingredients: Vec<QIngredient::Ref>,
    pub(crate) category_ids: Vec<i32>,
    pub(crate) tag_ids: Vec<i32>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: String,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16,
    pub(crate) season_ids: Vec<i32>,
    pub(crate) is_favorite: bool,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Patched {
    pub(crate) is_favorite: bool,
}

pub enum Filter {
    Search(String),
    Categories(Vec<i32>),
    Seasons(Vec<i32>),
    Ingredients(Vec<i32>),
    Tags(Vec<i32>),
}

impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            name: row.get("name"),
            notes: row.get("notes"),
            q_ingredients: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions"),
            n_shares: row.get("n_shares"),
            is_favorite: row.get("is_favorite"),
            seasons: Vec::new(),
        }
    }
}

impl From<&tokio_postgres::row::Row> for FromDBLight {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDBLight {
            id: row.get("id"),
            name: row.get("name"),
            image: row.get("image"),
            is_favorite: row.get("is_favorite"),
        }
    }
}

pub async fn get_many(
    db_conn: &Client,
    range: &Range,
    filters: &[Filter],
) -> Result<(Vec<FromDBLight>, i64), Error> {
    let mut query: String;

    query = String::from(
        "
        {CTEs}
        SELECT
            id,
            r.name,
            r.image,
            r.is_favorite,
            count(*) OVER() AS total_count
            {EXTRA_FIELDS}
        FROM recipes AS r
        {JOINS}
        ORDER BY {SORTING_FIELD}
        OFFSET $1
        LIMIT $2
    ",
    );

    let offset = range.from - 1;
    let limit = range.to - range.from + 1;
    let mut params: Vec<&(dyn ToSql + Sync)> = vec![&offset, &limit];

    let mut ctes = String::from("WITH dummy as (SELECT 1)");
    let mut extra_fields = String::new();
    let mut joins = String::new();
    let mut sorting_field = String::from("name");

    for f in filters {
        match f {
            Filter::Search(query) => {
                params.push(query);
                ctes.push_str(
                    format!(
                        "
                    , search_filter as (
                        SELECT
                            r.id,
                            AVG(w.word <<-> unaccent(r.name)) AS rank
                        FROM
                            ( SELECT UNNEST(STRING_TO_ARRAY(${}, ' ')) AS word ) AS w
                            CROSS JOIN
                            recipes AS r
                        GROUP BY r.id
                        HAVING MAX(w.word <<-> unaccent(r.name)) <= 0.4
                    )
                ",
                        params.len(),
                    )
                    .as_str(),
                );
                extra_fields.push_str(",sf.rank");
                joins.push_str(" INNER JOIN search_filter as sf USING (id)");
                sorting_field = String::from("sf.rank");
            }
            Filter::Categories(ids) => {
                params.push(ids);
                ctes.push_str(
                    format!(
                        "
                    , categ_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_categories
                        WHERE
                            category_id = any(${})
                    )
                ",
                        params.len(),
                    )
                    .as_str(),
                );
                joins.push_str(" INNER JOIN categ_filter USING (id)");
            }
            Filter::Seasons(ids) => {
                params.push(ids);
                ctes.push_str(
                    format!(
                        "
                    , season_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_seasons
                        WHERE
                            season_id = any(${})
                    )
                ",
                        //, season_filter as (
                        //    SELECT
                        //        recipe_id as id, array_agg(season_id) as sid
                        //    FROM
                        //        recipes_seasons
                        //    GROUP BY id
                        //    HAVING array_agg(season_id) @> ${}
                        params.len(),
                    )
                    .as_str(),
                );
                joins.push_str(" INNER JOIN season_filter USING (id)");
            }
            Filter::Ingredients(ids) => {
                params.push(ids);
                ctes.push_str(
                    format!(
                        "
                    , ingr_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_ingredients
                        WHERE
                            ingredient_id = any(${})
                    )
                ",
                        params.len(),
                    )
                    .as_str(),
                );
                joins.push_str(" INNER JOIN ingr_filter USING (id)");
            }
            Filter::Tags(ids) => {
                params.push(ids);
                ctes.push_str(
                    format!(
                        "
                    , tag_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_tags
                        WHERE
                            tag_id = any(${})
                    )
                ",
                        params.len(),
                    )
                    .as_str(),
                );
                joins.push_str(" INNER JOIN tag_filter USING (id)");
            }
            _ => (),
        }
    }

    query = query
        .replace("{CTEs}", ctes.as_str())
        .replace("{EXTRA_FIELDS}", extra_fields.as_str())
        .replace("{JOINS}", joins.as_str())
        .replace("{SORTING_FIELD}", sorting_field.as_str());

    trace!("{}", query);

    let rows = db_conn.query(&query, &params).await?;
    let total_count: i64 = rows.get(0).map(|r| r.get("total_count")).unwrap_or(0);
    let recipes: Vec<FromDBLight> = rows.iter().map(|r| r.into()).collect();

    Ok((recipes, total_count))
}

pub async fn add_one(db_conn: &mut Client, new_recipe: &New) -> Result<i32, Error> {
    let transaction = db_conn
        .transaction()
        .await
        .expect("Unable to start db transaction");
    let recipe_query = "
        INSERT INTO recipes
        (name, notes, preparation_time_min, cooking_time_min, image, instructions, n_shares)
        VALUES (sentence_case($1), $2, $3, $4, $5, $6, $7)
        RETURNING id;
    ";

    let new_id: i32 = transaction
        .query(
            recipe_query,
            &[
                &new_recipe.name,
                &new_recipe.notes,
                &new_recipe.prep_time_min,
                &new_recipe.cook_time_min,
                &new_recipe.image,
                &new_recipe.instructions,
                &new_recipe.n_shares,
            ],
        )
        .await
        .map(|rows| rows[0].get(0))?;

    // Tags
    if !new_recipe.tag_ids.is_empty() {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&new_id, &new_recipe.tag_ids];
        let insert_tags_query = "

            INSERT INTO recipes_tags
            (tag_id, recipe_id)
            SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id;
        ";
        transaction.execute(insert_tags_query, &args).await?;
    }

    // Categories
    if !new_recipe.category_ids.is_empty() {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&new_id, &new_recipe.category_ids];
        let insert_categories_query = "

            INSERT INTO recipes_categories
            (category_id, recipe_id)
            SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id;
        ";
        transaction.execute(insert_categories_query, &args).await?;
    }

    // Seasons
    if !new_recipe.season_ids.is_empty() {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&new_id, &new_recipe.season_ids];
        let insert_seasons_query = "

            INSERT INTO recipes_seasons
            (season_id, recipe_id)
            SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id;
        ";
        transaction.execute(insert_seasons_query, &args).await?;
    }

    // Ingredients
    if !new_recipe.q_ingredients.is_empty() {
        let mut ingr_ids: Vec<_> = Vec::new();
        let mut qtys: Vec<_> = Vec::new();
        let mut unit_ids: Vec<_> = Vec::new();

        new_recipe.q_ingredients.iter().for_each(|ref v| {
            ingr_ids.push(&v.id);
            qtys.push(&v.quantity);
            unit_ids.push(&v.unit_id);
        });
        let args: Vec<&(dyn ToSql + Sync)> = vec![&new_id, &ingr_ids, &qtys, &unit_ids];
        let insert_ingredients_query = "
            INSERT INTO recipes_ingredients
            (recipe_id, ingredient_id, quantity, unit_id)
            SELECT $1, ingredient_id, quantity, unit_id
            FROM
                UNNEST($2::int[], $3::real[], $4::int[])
                AS x(ingredient_id, quantity, unit_id)
        ";
        transaction.execute(insert_ingredients_query, &args).await?;
    }

    transaction
        .commit()
        .await
        .expect("Error when commiting transaction");
    Ok(new_id)
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let recipe_query = "
        SELECT
            id,
            name,
            notes,
            preparation_time_min,
            cooking_time_min,
            image,
            publication_date,
            instructions,
            n_shares,
            is_favorite
        FROM recipes
        WHERE id = $1
    ";

    let categories_query = "
        SELECT
            c.id,
            c.name
        FROM
            categories as c,
            recipes_categories as rc
        WHERE
            c.id = rc.category_id
            AND rc.recipe_id = $1
    ";

    let tags_query = "
        SELECT
            t.id,
            t.name
        FROM
            tags as t,
            recipes_tags as rt
        WHERE
            t.id = rt.tag_id
            AND rt.recipe_id = $1
    ";

    let seasons_query = "
        SELECT
            s.id,
            s.name
        FROM
            seasons as s,
            recipes_seasons as rs
        WHERE
            s.id = rs.season_id
            AND rs.recipe_id = $1
    ";

    let ingredients_query = "
        SELECT
            i.id as id,
            i.name as name,
            ri.quantity as quantity,
            u.id as unit_id,
            u.full_name as unit_full_name,
            u.short_name as unit_short_name
        FROM
            ingredients as i,
            recipes_ingredients as ri
            LEFT JOIN units as u
            ON ri.unit_id = u.id
        WHERE
            i.id = ri.ingredient_id
            AND ri.recipe_id = $1
            ";

    let mut maybe_recipe: Option<FromDB> = db_conn
        .query_opt(recipe_query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))?;

    if let Some(ref mut recipe) = maybe_recipe {
        let categories: Vec<category::FromDB> = db_conn
            .query(categories_query, &[&id])
            .await
            .map(|rows| rows.iter().map(|r| r.into()).collect())?;

        let tags: Vec<tag::FromDB> = db_conn
            .query(tags_query, &[&id])
            .await
            .map(|rows| rows.iter().map(|r| r.into()).collect())?;

        let seasons: Vec<season::FromDB> = db_conn
            .query(seasons_query, &[&id])
            .await
            .map(|rows| rows.iter().map(|r| r.into()).collect())?;

        let ingredients: Vec<QIngredient::Full> = db_conn
            .query(ingredients_query, &[&id])
            .await
            .map(|rows| rows.iter().map(|r| r.into()).collect())?;

        recipe.categories = categories;
        recipe.tags = tags;
        recipe.seasons = seasons;
        recipe.q_ingredients = ingredients;
    }

    Ok(maybe_recipe)
}

pub async fn modify_one(
    db_conn: &mut Client,
    id: i32,
    new_recipe: &New,
) -> Result<Option<()>, Error> {
    let transaction = db_conn
        .transaction()
        .await
        .expect("Unable to start db transaction");

    let recipe_query = "
        UPDATE recipes SET
            name = sentence_case($1),
            notes = $2,
            preparation_time_min = $3,
            cooking_time_min = $4,
            image = $5,
            instructions = $6,
            n_shares = $7,
            is_favorite = $8
        WHERE id = $9
        RETURNING id;
    ";
    transaction
        .query_opt(
            recipe_query,
            &[
                &new_recipe.name,
                &new_recipe.notes,
                &new_recipe.prep_time_min,
                &new_recipe.cook_time_min,
                &new_recipe.image,
                &new_recipe.instructions,
                &new_recipe.n_shares,
                &new_recipe.is_favorite,
                &id,
            ],
        )
        .await
        .map(|opt| opt.map(|_| ()))?; // OK(Some(row)) => Ok(Some(()))

    // Tags
    if new_recipe.tag_ids.is_empty() {
        let remove_tags_query = "
            DELETE FROM recipes_tags
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_tags_query, &[&id]).await?;
    } else {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&id, &new_recipe.tag_ids];
        let insert_tags_query = "

            INSERT INTO recipes_tags
            (tag_id, recipe_id)
            SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id
            ON CONFLICT DO NOTHING;
        ";
        transaction.execute(insert_tags_query, &args).await?;

        let remove_tags_query = "
            DELETE FROM recipes_tags
            WHERE
                recipe_id = $1
                AND tag_id <> ALL($2);
        ";
        transaction.execute(remove_tags_query, &args).await?;
    }

    // Seasons
    if new_recipe.season_ids.is_empty() {
        let remove_seasons_query = "
            DELETE FROM recipes_seasons
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_seasons_query, &[&id]).await?;
    } else {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&id, &new_recipe.season_ids];
        let insert_seasons_query = "

            INSERT INTO recipes_seasons
            (season_id, recipe_id)
            SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id
            ON CONFLICT DO NOTHING;
        ";
        transaction.execute(insert_seasons_query, &args).await?;

        let remove_seasons_query = "
            DELETE FROM recipes_seasons
            WHERE
                recipe_id = $1
                AND season_id <> ALL($2);
        ";
        transaction.execute(remove_seasons_query, &args).await?;
    }

    // Categories
    if new_recipe.category_ids.is_empty() {
        let remove_categories_query = "
            DELETE FROM recipes_categories
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_categories_query, &[&id]).await?;
    } else {
        let args: Vec<&(dyn ToSql + Sync)> = vec![&id, &new_recipe.category_ids];
        let insert_categories_query = "

            INSERT INTO recipes_categories
            (category_id, recipe_id)
            SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id
            ON CONFLICT DO NOTHING;
        ";
        transaction.execute(insert_categories_query, &args).await?;

        let remove_categories_query = "
            DELETE FROM recipes_categories
            WHERE
                recipe_id = $1
                AND category_id <> ALL($2);
        ";
        transaction.execute(remove_categories_query, &args).await?;
    }

    // Ingredients
    if new_recipe.q_ingredients.is_empty() {
        let remove_ingredients_query = "
            DELETE FROM recipes_ingredients
            WHERE recipe_id = $1;
        ";
        transaction
            .execute(remove_ingredients_query, &[&id])
            .await?;
    } else {
        let mut ingr_ids: Vec<_> = Vec::new();
        let mut qtys: Vec<_> = Vec::new();
        let mut unit_ids: Vec<_> = Vec::new();

        new_recipe.q_ingredients.iter().for_each(|ref v| {
            ingr_ids.push(&v.id);
            qtys.push(&v.quantity);
            unit_ids.push(&v.unit_id);
        });
        let args: Vec<&(dyn ToSql + Sync)> = vec![&id, &ingr_ids, &qtys, &unit_ids];
        let insert_ingredients_query = "
            INSERT INTO recipes_ingredients
            (recipe_id, ingredient_id, quantity, unit_id)
            SELECT $1, ingredient_id, quantity, unit_id
            FROM
                UNNEST($2::int[], $3::real[], $4::int[])
                AS x(ingredient_id, quantity, unit_id)
            ON CONFLICT DO NOTHING;
        ";
        transaction.execute(insert_ingredients_query, &args).await?;

        let args: Vec<&(dyn ToSql + Sync)> = vec![&id, &ingr_ids];
        let remove_ingredients_query = "
            DELETE FROM recipes_ingredients
            WHERE
                recipe_id = $1
                AND ingredient_id <> ALL($2);
        ";
        transaction.execute(remove_ingredients_query, &args).await?;
    }

    transaction
        .commit()
        .await
        .expect("Error when commiting transaction");
    Ok(Some(()))
}

pub async fn patch_one(
    db_conn: &Client,
    id: i32,
    patched_recipe: &Patched,
) -> Result<Option<()>, Error> {
    let patch_query = "
        UPDATE recipes
        SET is_favorite = $1
        WHERE id = $2
        RETURNING id;
    ";
    db_conn
        .query_opt(patch_query, &[&patched_recipe.is_favorite, &id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}

pub async fn delete_one(db_conn: &Client, id: i32) -> Result<Option<()>, Error> {
    let delete_query = "
        DELETE FROM recipes
        WHERE id = $1
        RETURNING id;
    ";
    db_conn
        .query_opt(delete_query, &[&id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}
