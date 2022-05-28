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
    pub(crate) q_ingredient_ids: Vec<QIngredient::Ref>,
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
            q_ingredients: row.get::<_, types::Json<_>>("ingredients").0,
            categories: row.get::<_, types::Json<_>>("categories").0,
            tags: row.get::<_, types::Json<_>>("tags").0,
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions"),
            n_shares: row.get("n_shares"),
            is_favorite: row.get("is_favorite"),
            seasons: row.get::<_, types::Json<_>>("seasons").0,
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
        let values_query_params = gen_sql_query_params(new_recipe.tag_ids.len(), 2);
        let tags_query = format!(
            "
            INSERT INTO recipes_tags
            (tag_id, recipe_id)
            VALUES {};
        ",
            values_query_params
        );

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for tag_id in &new_recipe.tag_ids {
            flat_values.extend_from_slice(&[tag_id, &new_id]);
        }

        transaction
            .execute(tags_query.as_str(), &flat_values)
            .await?;
    }

    // Categories
    if !new_recipe.category_ids.is_empty() {
        let values_query_params = gen_sql_query_params(new_recipe.category_ids.len(), 2);
        let categories_query = format!(
            "
            INSERT INTO recipes_categories
            (category_id, recipe_id)
            VALUES {};
        ",
            values_query_params
        );

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for category_id in &new_recipe.category_ids {
            flat_values.extend_from_slice(&[category_id, &new_id]);
        }

        transaction
            .execute(categories_query.as_str(), &flat_values)
            .await?;
    }
    //
    // Ingredients
    if !new_recipe.q_ingredient_ids.is_empty() {
        let values_query_params = gen_sql_query_params(new_recipe.q_ingredient_ids.len(), 4);
        let ingredients_query = format!(
            "
            INSERT INTO recipes_ingredients
            (recipe_id, ingredient_id, quantity, unit_id)
            VALUES {};
        ",
            values_query_params
        );

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for ingr in &new_recipe.q_ingredient_ids {
            flat_values.extend_from_slice(&[&new_id, &ingr.id, &ingr.quantity, &ingr.unit_id]);
        }

        transaction
            .execute(ingredients_query.as_str(), &flat_values)
            .await?;
    }

    transaction
        .commit()
        .await
        .expect("Error when commiting transaction");
    Ok(new_id)
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let recipe_query = "
        WITH r_seasons AS (
            SELECT
                s.id,
                s.name
            FROM
                seasons AS s INNER JOIN recipes_seasons AS rs
                ON s.id = rs.season_id
            WHERE rs.recipe_id = $1
        ),
        r_tags AS (
            SELECT
                t.id,
                t.name
            FROM
                tags AS t INNER JOIN recipes_tags AS rt
                ON t.id = rt.tag_id
            WHERE rt.recipe_id = $1
        ),
        r_categories AS (
            SELECT
                c.id,
                c.name
            FROM
                categories AS c INNER JOIN recipes_categories AS rc
                ON c.id = rc.category_id
            WHERE rc.recipe_id = $1
        ),
        r_units AS (
            SELECT
                u.id,
                u.short_name,
                u.full_name
            FROM
                units AS u
            WHERE u.id in (SELECT unit_id from recipes_ingredients where recipe_id = $1)
        ),
        r_ingredients AS (
            SELECT
                i.id,
                i.name,
                ri.quantity,
                row_to_json(u) as unit
            FROM
                ingredients AS i INNER JOIN recipes_ingredients AS ri
                ON i.id = ri.ingredient_id
                LEFT JOIN (select * from r_units) as u
                ON u.id = ri.unit_id
            WHERE ri.recipe_id = $1
        ),

        r_seasons_json AS (
            SELECT
                COALESCE(json_agg(row_to_json(s)), '[]'::json) AS seasons
            FROM (SELECT * FROM r_seasons) AS s
        ),
        r_tags_json AS (
            SELECT
                COALESCE(json_agg(row_to_json(t)), '[]'::json) AS tags
            FROM (SELECT * FROM r_tags) AS t
        ),
        r_categories_json AS (
            SELECT
                COALESCE(json_agg(row_to_json(c)), '[]'::json) AS categories
            FROM (SELECT * FROM r_categories) AS c
        ),
        r_ingredients_json AS (
            SELECT
                COALESCE(json_agg(row_to_json(i)), '[]'::json) AS ingredients
            FROM (SELECT * FROM r_ingredients) AS i
        )

        SELECT
            r.id,
            r.name,
            r.notes,
            r.preparation_time_min,
            r.cooking_time_min,
            r.image,
            r.publication_date,
            r.instructions,
            r.n_shares,
            r.is_favorite,
            seasons,
            tags,
            categories,
            ingredients
        FROM
            recipes as r,
            r_seasons_json,
            r_tags_json,
            r_categories_json,
            r_ingredients_json
        WHERE r.id = $1;
    ";

    let maybe_recipe: Option<FromDB> = db_conn
        .query_opt(recipe_query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))?;

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

    //TODO this is ugly and should be more simple

    // Tags
    if new_recipe.tag_ids.is_empty() {
        let remove_tags_query = "
            DELETE FROM recipes_tags
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_tags_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.tag_ids.len(), 2);
        let insert_tags_query = format!(
            "
            INSERT INTO recipes_tags
            (tag_id, recipe_id)
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            query_params
        );

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for tag_id in &new_recipe.tag_ids {
            query_args.extend_from_slice(&[tag_id, &id]);
        }
        transaction
            .execute(insert_tags_query.as_str(), &query_args)
            .await?;

        let query_params = gen_sql_query_params_from(new_recipe.tag_ids.len(), 1, 2);
        let remove_tags_query = format!(
            "
            DELETE FROM recipes_tags
            WHERE
                recipe_id = $1
                AND tag_id NOT IN ({});
        ",
            query_params
        );
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for tag_id in &new_recipe.tag_ids {
            query_args.push(tag_id);
        }
        transaction
            .execute(remove_tags_query.as_str(), &query_args)
            .await?;
    }

    // Seasons
    if new_recipe.season_ids.is_empty() {
        let remove_seasons_query = "
            DELETE FROM recipes_seasons
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_seasons_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.season_ids.len(), 2);
        let insert_seasons_query = format!(
            "
            INSERT INTO recipes_seasons
            (season_id, recipe_id)
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            query_params
        );

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for season_id in &new_recipe.season_ids {
            query_args.extend_from_slice(&[season_id, &id]);
        }
        transaction
            .execute(insert_seasons_query.as_str(), &query_args)
            .await?;

        let query_params = gen_sql_query_params_from(new_recipe.season_ids.len(), 1, 2);
        let remove_seasons_query = format!(
            "
            DELETE FROM recipes_seasons
            WHERE
                recipe_id = $1
                AND season_id NOT IN ({});
        ",
            query_params
        );
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for season_id in &new_recipe.season_ids {
            query_args.push(season_id);
        }
        transaction
            .execute(remove_seasons_query.as_str(), &query_args)
            .await?;
    }

    // Categories
    if new_recipe.category_ids.is_empty() {
        let remove_categories_query = "
            DELETE FROM recipes_categories
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_categories_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.category_ids.len(), 2);
        let insert_categories_query = format!(
            "
            INSERT INTO recipes_categories
            (category_id, recipe_id)
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            query_params
        );

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for category_id in &new_recipe.category_ids {
            query_args.extend_from_slice(&[category_id, &id]);
        }
        transaction
            .execute(insert_categories_query.as_str(), &query_args)
            .await?;

        let query_params = gen_sql_query_params_from(new_recipe.category_ids.len(), 1, 2);
        let remove_categories_query = format!(
            "
            DELETE FROM recipes_categories
            WHERE
                recipe_id = $1
                AND category_id NOT IN ({});
        ",
            query_params
        );
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for category_id in &new_recipe.category_ids {
            query_args.push(category_id);
        }
        transaction
            .execute(remove_categories_query.as_str(), &query_args)
            .await?;
    }

    // Ingredients
    if new_recipe.q_ingredient_ids.is_empty() {
        let remove_ingredients_query = "
            DELETE FROM recipes_ingredients
            WHERE recipe_id = $1;
        ";
        transaction
            .execute(remove_ingredients_query, &[&id])
            .await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.q_ingredient_ids.len(), 4);
        let insert_ingredients_query = format!(
            "
            INSERT INTO recipes_ingredients
            (recipe_id, ingredient_id, quantity, unit_id)
            VALUES {}
            ON CONFLICT DO NOTHING;
        ",
            query_params
        );

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for ingr in &new_recipe.q_ingredient_ids {
            query_args.extend_from_slice(&[&id, &ingr.id, &ingr.quantity, &ingr.unit_id]);
        }
        transaction
            .execute(insert_ingredients_query.as_str(), &query_args)
            .await?;

        let query_params = gen_sql_query_params_from(new_recipe.q_ingredient_ids.len(), 1, 2);
        let remove_ingredients_query = format!(
            "
            DELETE FROM recipes_ingredients
            WHERE
                recipe_id = $1
                AND ingredient_id NOT IN ({});
        ",
            query_params
        );
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for ingr in &new_recipe.q_ingredient_ids {
            query_args.push(&ingr.id);
        }
        transaction
            .execute(remove_ingredients_query.as_str(), &query_args)
            .await?;
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
