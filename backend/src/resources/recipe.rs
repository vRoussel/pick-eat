use super::{account, category, diet, ingredient, qingredient, season, tag};
use crate::query_params::Range;
use log::trace;
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgConnection, PgRow};
use sqlx::{query, query_as, query_unchecked, Connection, Error, FromRow, Row};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    id: i32,
    name: String,
    notes: String,
    q_ingredients: Vec<qingredient::FromDB>,
    categories: Vec<category::FromDB>,
    tags: Vec<tag::FromDB>,
    prep_time_min: i16,
    cook_time_min: i16,
    image: String,
    publish_date: time::Date,
    instructions: Vec<String>,
    n_shares: i16,
    is_favorite: bool,
    seasons: Vec<season::FromDB>,
    author: account::FromDBPublic,
    diets: Vec<diet::FromDB>,
}

#[derive(Debug, Serialize)]
pub struct FromDBLight {
    id: i32,
    name: String,
    q_ingredients: Vec<qingredient::FromDB>,
    image: String,
    n_shares: i16,
    is_favorite: bool,
    diets: Vec<diet::FromDB>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    name: String,
    notes: String,
    q_ingredients: Vec<qingredient::Ref>,
    category_ids: Vec<category::Ref>,
    tag_ids: Vec<tag::Ref>,
    prep_time_min: i16,
    cook_time_min: i16,
    image: String,
    instructions: Vec<String>,
    n_shares: i16,
    season_ids: Vec<season::Ref>,
    diet_ids: Vec<diet::Ref>,
}

pub enum Filter {
    Search(String),
    Categories(Vec<category::Ref>),
    Seasons(Vec<season::Ref>),
    Ingredients(Vec<ingredient::Ref>),
    Tags(Vec<tag::Ref>),
    Account(i32),
    Diets(Vec<diet::Ref>),
}

pub enum SortMethod {
    Random { seed: i32 },
}

impl FromRow<'_, PgRow> for FromDB {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let _tags: sqlx::types::Json<_> = row.get("tags");
        let _categories: sqlx::types::Json<_> = row.get("categories");
        let _ingredients: sqlx::types::Json<_> = row.get("ingredients");
        let _seasons: sqlx::types::Json<_> = row.get("seasons");
        let _diets: sqlx::types::Json<_> = row.get("diets");
        let _author = account::FromDBPublic {
            id: row.get("author_id"),
            display_name: row.get("author_name"),
        };
        Ok(FromDB {
            id: row.get("id"),
            name: row.get("name"),
            notes: row.get("notes"),
            q_ingredients: _ingredients.0,
            categories: _categories.0,
            tags: _tags.0,
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions"),
            n_shares: row.get("n_shares"),
            is_favorite: row.get("is_favorite"),
            seasons: _seasons.0,
            author: _author,
            diets: _diets.0,
        })
    }
}

impl FromRow<'_, PgRow> for FromDBLight {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        let _ingredients: sqlx::types::Json<_> = row.get("ingredients");
        let _diets: sqlx::types::Json<_> = row.get("diets");
        Ok(FromDBLight {
            id: row.get("id"),
            name: row.get("name"),
            q_ingredients: _ingredients.0,
            image: row.get("image"),
            n_shares: row.get("n_shares"),
            is_favorite: row.get("is_favorite"),
            diets: _diets.0,
        })
    }
}

pub async fn get_many(
    db_conn: &mut PgConnection,
    range: &Range,
    filters: &[Filter],
    sort_method: &SortMethod,
    account_id: Option<i32>,
) -> Result<(Vec<FromDBLight>, i64), Error> {
    let mut builder = sqlx::QueryBuilder::new("");
    let mut joins = String::new();
    let mut sorting_fields: Vec<String> = Vec::new();

    builder.push("WITH dummy as (SELECT 1)");

    // Filters
    for f in filters {
        match f {
            Filter::Search(query) => {
                builder
                    .push(
                        "
                        , search_filter as (
                            SELECT
                                r.id,
                                AVG(w.word <<-> unaccent(r.name)) AS rank
                            FROM
                                ( SELECT UNNEST(STRING_TO_ARRAY(
                        ",
                    )
                    .push_bind(query)
                    .push(
                        "
                        , ' ')) AS word ) AS w
                            CROSS JOIN
                            recipes AS r
                        GROUP BY r.id
                        HAVING MAX(w.word <<-> unaccent(r.name)) <= 0.4
                        )
                        ",
                    );
                joins.push_str(" INNER JOIN search_filter as sf USING (id)");
                sorting_fields.push(String::from("sf.rank"));
            }
            Filter::Categories(ids) => {
                builder
                    .push(
                        "
                        , categ_filter as (
                            SELECT
                                distinct(recipe_id) as id
                            FROM
                                recipes_categories
                            WHERE
                                category_id = any(
                        ",
                    )
                    .push_bind(ids)
                    .push("))");
                joins.push_str(" INNER JOIN categ_filter USING (id)");
            }
            Filter::Seasons(ids) => {
                builder
                    .push(
                        "
                        , season_filter as (
                            SELECT
                                distinct(recipe_id) as id
                            FROM
                                recipes_seasons
                            WHERE
                                season_id = any(
                        ",
                    )
                    .push_bind(ids)
                    .push("))");
                joins.push_str(" INNER JOIN season_filter USING (id)");
            }
            Filter::Ingredients(ids) => {
                builder
                    .push(
                        "
                        , ingr_filter as (
                            SELECT
                                distinct(recipe_id) as id
                            FROM
                                recipes_ingredients
                            WHERE
                                ingredient_id = any(
                        ",
                    )
                    .push_bind(ids)
                    .push("))");
                joins.push_str(" INNER JOIN ingr_filter USING (id)");
            }
            Filter::Tags(ids) => {
                builder
                    .push(
                        "
                        , tag_filter as (
                            SELECT
                                distinct(recipe_id) as id
                            FROM
                                recipes_tags
                            WHERE
                                tag_id = any(
                        ",
                    )
                    .push_bind(ids)
                    .push("))");
                joins.push_str(" INNER JOIN tag_filter USING (id)");
            }
            Filter::Account(id) => {
                builder
                    .push(
                        "
                        , account_filter as (
                            SELECT
                                distinct(id) as id
                            FROM
                                recipes
                            WHERE
                                author_id = 
                        ",
                    )
                    .push_bind(id)
                    .push(")");
                joins.push_str(" INNER JOIN account_filter USING (id)");
            }
            Filter::Diets(ids) => {
                builder
                    .push(
                        "
                        , diet_filter as (
                            SELECT
                                distinct(recipe_id) as id
                            FROM
                                recipes_diets
                            WHERE
                                diet_id = any(
                        ",
                    )
                    .push_bind(ids)
                    .push("))");
                joins.push_str(" INNER JOIN diet_filter USING (id)");
            }
        };
    }

    match sort_method {
        SortMethod::Random { seed } => {
            sorting_fields.push(format!(
                "(extract(epoch from publication_date)+id) % {}",
                seed
            ));
        }
    };

    let offset = range.from - 1;
    let limit = range.to - range.from + 1;

    builder
        .push(
            "
            SELECT
                id,
                r.name,
                r.image,
                fav IS NOT null as is_favorite,
                get_ingredients_json(id) as ingredients,
                get_diets_json(id) as diets,
                n_shares,
                count(*) OVER() AS total_count
            FROM recipes AS r
            LEFT JOIN accounts_fav_recipes fav
                ON r.id = fav.recipe_id AND fav.account_id = ",
        )
        .push_bind(account_id)
        .push(" \n")
        .push(joins)
        .push(
            "
        ORDER BY ",
        )
        .push(sorting_fields.join(", "))
        .push(" OFFSET ")
        .push_bind(offset)
        .push(" LIMIT ")
        .push_bind(limit);

    // Intersection filter instead of union
    //, season_filter as (
    //    SELECT
    //        recipe_id as id, array_agg(season_id) as sid
    //    FROM
    //        recipes_seasons
    //    GROUP BY id
    //    HAVING array_agg(season_id) @> ${}

    trace!("{}", builder.sql());

    let rows: Vec<PgRow> = builder.build().fetch_all(db_conn).await?;
    if rows.is_empty() {
        return Ok((vec![], 0));
    }
    let total_count: i64 = rows.iter().peekable().peek().unwrap().get("total_count");
    let recipes: Vec<FromDBLight> = rows
        .into_iter()
        .map(|r| FromDBLight::from_row(&r))
        .collect::<Result<_, _>>()?;
    Ok((recipes, total_count))
}

pub async fn add_one(
    db_conn: &mut PgConnection,
    new_recipe: &New,
    user_id: i32,
) -> Result<i32, Error> {
    let mut transaction = db_conn.begin().await?;
    let new_id: i32 = query!(
        "
            INSERT INTO recipes
            (name, notes, preparation_time_min, cooking_time_min, image, instructions, n_shares, author_id)
            VALUES (sentence_case($1), $2, $3, $4, $5, $6, $7, $8)
            RETURNING id;
        ",
        new_recipe.name,
        new_recipe.notes,
        new_recipe.prep_time_min,
        new_recipe.cook_time_min,
        new_recipe.image,
        &new_recipe.instructions,
        new_recipe.n_shares,
        user_id
    )
    .fetch_one(&mut transaction)
    .await?
    .id;

    // Tags
    if !new_recipe.tag_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_tags
                (tag_id, recipe_id)
                SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id;
            ",
            new_id,
            &new_recipe.tag_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    // Categories
    if !new_recipe.category_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_categories
                (category_id, recipe_id)
                SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id;
            ",
            new_id,
            &new_recipe.category_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    // Seasons
    if !new_recipe.season_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_seasons
                (season_id, recipe_id)
                SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id;
            ",
            new_id,
            &new_recipe.season_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    // Diets
    if !new_recipe.diet_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_diets
                (diet_id, recipe_id)
                SELECT diet_id, $1 FROM UNNEST($2::int[]) as diet_id;
            ",
            new_id,
            &new_recipe.diet_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    // Ingredients
    if !new_recipe.q_ingredients.is_empty() {
        let mut ingr_ids: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());
        let mut qtys: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());
        let mut unit_ids: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());

        new_recipe.q_ingredients.iter().for_each(|ref v| {
            ingr_ids.push(v.id);
            qtys.push(v.quantity);
            unit_ids.push(v.unit_id);
        });
        // We have to use query_unchecked here since $3 and $4 are &[Option<_>] and sqlx wants &[_]
        // see https://github.com/launchbadge/sqlx/issues/1893
        query_unchecked!(
            r#"
                INSERT INTO recipes_ingredients
                (recipe_id, ingredient_id, quantity, unit_id)
                SELECT $1, ingredient_id, quantity, unit_id
                FROM
                    UNNEST($2::int[], $3::real[], $4::int[])
                    AS x(ingredient_id, quantity, unit_id)
            "#,
            new_id,
            &ingr_ids,
            &qtys,
            &unit_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(new_id)
}

pub async fn get_one(
    db_conn: &mut PgConnection,
    id: i32,
    account_id: Option<i32>,
) -> Result<Option<FromDB>, Error> {
    let recipe: Option<FromDB> = query_as(
        r#"
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
                fav IS NOT null as is_favorite,
                accounts.id as author_id,
                accounts.display_name as author_name,
                get_categories_json(r.id) as categories,
                get_ingredients_json(r.id) as ingredients,
                get_tags_json(r.id) as tags,
                get_seasons_json(r.id) as seasons,
                get_diets_json(r.id) as diets
            FROM
                recipes r
                LEFT JOIN accounts_fav_recipes fav
                    ON r.id = fav.recipe_id AND fav.account_id = $2
                INNER JOIN accounts
                    ON accounts.id = r.author_id
            WHERE r.id = $1
        "#,
    )
    .bind(id)
    .bind(account_id)
    .fetch_optional(db_conn)
    .await?;
    Ok(recipe)
}

pub async fn get_author_id(db_conn: &mut PgConnection, id: i32) -> Result<i32, Error> {
    let author_id: i32 = query!(
        "
            SELECT author_id
            FROM recipes
            WHERE id = $1
        ",
        id
    )
    .fetch_one(db_conn)
    .await?
    .author_id;
    Ok(author_id)
}

pub async fn modify_one(
    db_conn: &mut PgConnection,
    id: i32,
    new_recipe: &New,
) -> Result<Option<()>, Error> {
    let mut transaction = db_conn.begin().await?;

    let n_rows: u64 = query!(
        "
            UPDATE recipes SET
                name = sentence_case($1),
                notes = $2,
                preparation_time_min = $3,
                cooking_time_min = $4,
                image = $5,
                instructions = $6,
                n_shares = $7
            WHERE id = $8
        ",
        new_recipe.name,
        new_recipe.notes,
        new_recipe.prep_time_min,
        new_recipe.cook_time_min,
        new_recipe.image,
        &new_recipe.instructions,
        new_recipe.n_shares,
        id
    )
    .execute(&mut transaction)
    .await?
    .rows_affected();

    if n_rows <= 0 {
        return Ok(None);
    }

    // Tags
    if !new_recipe.tag_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_tags
                (tag_id, recipe_id)
                SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.tag_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    query!(
        "
                DELETE FROM recipes_tags
                WHERE
                    recipe_id = $1
                    AND tag_id <> ALL($2);
            ",
        id,
        &new_recipe.tag_ids
    )
    .execute(&mut transaction)
    .await?;

    // Seasons
    if !new_recipe.season_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_seasons
                (season_id, recipe_id)
                SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.season_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    query!(
        "
                DELETE FROM recipes_seasons
                WHERE
                    recipe_id = $1
                    AND season_id <> ALL($2);
            ",
        id,
        &new_recipe.season_ids
    )
    .execute(&mut transaction)
    .await?;

    // Categories
    if !new_recipe.category_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_categories
                (category_id, recipe_id)
                SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.category_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    query!(
        "
                DELETE FROM recipes_categories
                WHERE
                    recipe_id = $1
                    AND category_id <> ALL($2);
            ",
        id,
        &new_recipe.category_ids
    )
    .execute(&mut transaction)
    .await?;

    // Diets
    if !new_recipe.diet_ids.is_empty() {
        query!(
            "
                INSERT INTO recipes_diets
                (diet_id, recipe_id)
                SELECT diet_id, $1 FROM UNNEST($2::int[]) as diet_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.diet_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    query!(
        "
                DELETE FROM recipes_diets
                WHERE
                    recipe_id = $1
                    AND diet_id <> ALL($2);
            ",
        id,
        &new_recipe.diet_ids
    )
    .execute(&mut transaction)
    .await?;

    // Ingredients
    let mut ingr_ids: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());
    let mut qtys: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());
    let mut unit_ids: Vec<_> = Vec::with_capacity(new_recipe.q_ingredients.len());

    new_recipe.q_ingredients.iter().for_each(|ref v| {
        ingr_ids.push(v.id);
        qtys.push(v.quantity);
        unit_ids.push(v.unit_id);
    });

    if !new_recipe.q_ingredients.is_empty() {
        query_unchecked!(
            "
                WITH input AS (
                    SELECT i_id, q, u_id
                    FROM
                        UNNEST($2::int[], $3::real[], $4::int[])
                        AS x(i_id, q, u_id)
                )
                INSERT INTO recipes_ingredients as ri
                (recipe_id, ingredient_id, quantity, unit_id)
                SELECT $1, i_id, q, u_id
                FROM input
                ON CONFLICT(recipe_id, ingredient_id) DO UPDATE
                    SET quantity = EXCLUDED.quantity, unit_id = EXCLUDED.unit_id
                    WHERE ri.recipe_id = $1 AND ri.ingredient_id = EXCLUDED.ingredient_id
                ;
            ",
            id,
            &ingr_ids,
            &qtys,
            &unit_ids
        )
        .execute(&mut transaction)
        .await?;
    }

    query!(
        "
                DELETE FROM recipes_ingredients
                WHERE
                    recipe_id = $1
                    AND ingredient_id <> ALL($2);
            ",
        id,
        &ingr_ids
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn delete_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, Error> {
    query!(
        "
            DELETE FROM recipes
            WHERE id = $1
        ",
        id
    )
    .execute(db_conn)
    .await?;
    Ok(Some(()))
}
