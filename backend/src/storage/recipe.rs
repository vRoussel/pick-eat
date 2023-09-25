use std::convert::TryFrom;

use super::{begin_transaction, DBConstraint, IsolationLevel, StorageError};
use crate::models::{
    Category, Diet, InvalidRecipe, InvalidityKind, NewRecipe, QIngredient, Range, Recipe,
    RecipeFilters, RecipeSummary, Season, SortMethod, Tag,
};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as, query_unchecked, Connection};
use sqlx_conditional_queries::conditional_query_as;

impl<'a> TryFrom<&DBConstraint> for InvalidRecipe {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "recipes_ck_times" => Ok(InvalidRecipe {
                cooking_time_min: Some(InvalidityKind::BadValue),
                preparation_time_min: Some(InvalidityKind::BadValue),
                ..Default::default()
            }),
            "recipes_fk_author_id" => Ok(InvalidRecipe {
                author_id: Some(InvalidityKind::InvalidRef),
                ..Default::default()
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn get_many_recipes(
    db_conn: &mut PgConnection,
    range: &Range,
    filters: &RecipeFilters,
    sort_method: SortMethod,
    account_id: Option<i32>,
) -> Result<Vec<RecipeSummary>, StorageError> {
    let offset = range.from - 1;
    let limit = range.to - range.from + 1;
    let recipes: Vec<RecipeSummary> = conditional_query_as!(
        RecipeSummary,
        r#"
            WITH
                dummy as (SELECT 1)
                {#search_filter}
                {#categ_filter}
                {#season_filter}
                {#tag_filter}
                {#diet_filter}
                {#ingredient_filter}
                {#account_filter}
            SELECT
                r.id,
                r.name,
                r.image,
                is_recipe_in_account_favs(r.id, {account_id}) as "is_favorite!",
                r.ingredients as "ingredients: Vec<QIngredient>",
                r.diets as "diets: Vec<Diet>",
                n_shares,
                is_private,
                count(*) OVER() AS "total_count!"
            FROM recipes_full AS r
                {#search_filter_join}
                {#categ_filter_join}
                {#season_filter_join}
                {#tag_filter_join}
                {#diet_filter_join}
                {#ingredient_filter_join}
                {#account_filter_join}
            WHERE
                is_private = 'f' OR author_id = {account_id}
            ORDER BY
                {#search_filter_order}
                {#ingredient_filter_order}
                {#tag_filter_order}
                {#base_order}
            OFFSET {offset}
            LIMIT {limit}
        "#,
        #(search_filter, search_filter_join, search_filter_order) = match &filters.search {
            None => ("","",""),
            Some(query) => (
                "
                    , search_filter as (
                        SELECT
                            r.id,
                            AVG(w.word <<-> unaccent(r.name)) AS rank
                        FROM
                        (
                            SELECT UNNEST(
                                STRING_TO_ARRAY(
                                    unaccent({query})
                                    , ' '
                                )
                            ) as word
                        ) as w
                        CROSS JOIN recipes as r
                        GROUP BY r.id
                        HAVING MAX(w.word <<-> unaccent(r.name)) <= 0.7
                    )
                ",
                " INNER JOIN search_filter as sf USING (id)",
                " sf.rank, "
            )
        },
        #(categ_filter, categ_filter_join) = match &filters.categories {
            None => ("",""),
            Some(categ_ids) => (
                "
                    , categ_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_categories
                        WHERE category_id = any({categ_ids})
                    )
                ",
                " INNER JOIN categ_filter USING (id)",
            )
        },
        #(season_filter, season_filter_join) = match &filters.seasons {
            None => ("",""),
            Some(season_ids) => (
                "
                    , season_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_seasons
                        WHERE season_id = any({season_ids})
                    )
                ",
                " INNER JOIN season_filter USING (id)",
            )
        },
        #(tag_filter, tag_filter_join, tag_filter_order) = match &filters.tags {
            None => ("","",""),
            Some(tag_ids) => (
                "
                    , tag_filter as (
                        SELECT
                            recipe_id as id,
                            count(*) as rank
                        FROM
                            recipes_tags
                        WHERE tag_id = any({tag_ids})
                        GROUP BY id
                    )
                ",
                " INNER JOIN tag_filter as tf USING (id)",
                "tf.rank DESC, "
            )
        },
        #(diet_filter, diet_filter_join) = match &filters.diets {
            None => ("",""),
            Some(diet_ids) => (
                "
                    , diet_filter as (
                        SELECT
                            distinct(recipe_id) as id
                        FROM
                            recipes_diets
                        WHERE diet_id = any({diet_ids})
                    )
                ",
                " INNER JOIN diet_filter USING (id)",
            )
        },
        #(ingredient_filter, ingredient_filter_join, ingredient_filter_order) = match &filters.ingredients {
            None => ("","",""),
            Some(ingredient_ids) => (
                "
                    , ingredient_filter as (
                        SELECT
                            recipe_id as id,
                            count(*) as rank
                        FROM
                            recipes_ingredients
                        WHERE ingredient_id = any({ingredient_ids})
                        GROUP BY id
                    )
                ",
                " INNER JOIN ingredient_filter as if USING (id)",
                " if.rank DESC, "
            )
        },
        #(account_filter, account_filter_join) = match &filters.account {
            None => ("",""),
            Some(account_id) => (
                "
                    , account_filter as (
                        SELECT
                            distinct(id)
                        FROM
                            recipes
                        WHERE author_id = {account_id}
                    )
                ",
                " INNER JOIN account_filter USING (id)",
            )
        },
        #base_order = match sort_method {
            SortMethod::Random { seed } =>
                "(extract(epoch from publication_date)+id)::bigint % {seed: i64}"
        }
    )
    .fetch_all(db_conn)
    .await?;

    Ok(recipes)
}

pub async fn add_recipe(
    db_conn: &mut PgConnection,
    new_recipe: &NewRecipe,
    user_id: i32,
) -> Result<i32, StorageError> {
    let mut transaction = begin_transaction(db_conn, IsolationLevel::Default).await?;
    let new_id: i32 = query!(
        "
            INSERT INTO recipes
            (name, notes, preparation_time_min, cooking_time_min, image, instructions, n_shares, is_private, author_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id;
        ",
        new_recipe.name,
        new_recipe.notes,
        new_recipe.preparation_time_min,
        new_recipe.cooking_time_min,
        new_recipe.image,
        &new_recipe.instructions,
        new_recipe.n_shares,
        new_recipe.is_private,
        user_id
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    // Tags
    if !new_recipe.tags.is_empty() {
        query!(
            "
                INSERT INTO recipes_tags
                (tag_id, recipe_id)
                SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id;
            ",
            new_id,
            &new_recipe.tags
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Categories
    if !new_recipe.categories.is_empty() {
        query!(
            "
                INSERT INTO recipes_categories
                (category_id, recipe_id)
                SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id;
            ",
            new_id,
            &new_recipe.categories
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Seasons
    if !new_recipe.seasons.is_empty() {
        query!(
            "
                INSERT INTO recipes_seasons
                (season_id, recipe_id)
                SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id;
            ",
            new_id,
            &new_recipe.seasons
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Diets
    if !new_recipe.diets.is_empty() {
        query!(
            "
                INSERT INTO recipes_diets
                (diet_id, recipe_id)
                SELECT diet_id, $1 FROM UNNEST($2::int[]) as diet_id;
            ",
            new_id,
            &new_recipe.diets
        )
        .execute(&mut *transaction)
        .await?;
    }

    // Ingredients
    if !new_recipe.ingredients.is_empty() {
        let mut ingr_ids: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());
        let mut qtys: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());
        let mut unit_ids: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());

        new_recipe.ingredients.iter().for_each(|ref v| {
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
        .execute(&mut *transaction)
        .await?;
    }

    transaction.commit().await?;

    Ok(new_id)
}

pub async fn get_recipes_count(db_conn: &mut PgConnection) -> Result<i64, StorageError> {
    let total_count = query!(r#"SELECT count(*) as "count!" FROM recipes"#)
        .fetch_one(db_conn)
        .await?
        .count;
    Ok(total_count)
}

pub async fn get_recipe_author(
    db_conn: &mut PgConnection,
    recipe_id: i32,
) -> Result<i32, StorageError> {
    let author_id = query!(r#"SELECT author_id FROM recipes WHERE id = $1"#, recipe_id)
        .fetch_one(db_conn)
        .await?
        .author_id;
    Ok(author_id)
}

pub async fn get_recipe_by_id(
    db_conn: &mut PgConnection,
    recipe_id: i32,
    account_id: Option<i32>,
) -> Result<Option<Recipe>, StorageError> {
    let recipe: Option<Recipe> = query_as!(
        Recipe,
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
                r.is_private,
                r.author_id,
                is_recipe_in_account_favs(r.id, $1) as "is_favorite!",
                a.display_name as author_name,
                r.tags as "tags: Vec<Tag>",
                r.categories as "categories: Vec<Category>",
                r.diets as "diets: Vec<Diet>",
                r.seasons as "seasons: Vec<Season>",
                r.ingredients as "ingredients: Vec<QIngredient>"
            FROM
                recipes_full r
                INNER JOIN accounts a
                    ON a.id = r.author_id
            WHERE r.id = $2
        "#,
        account_id,
        recipe_id,
    )
    .fetch_optional(db_conn)
    .await?;
    Ok(recipe)
}

pub async fn replace_recipe(
    db_conn: &mut PgConnection,
    id: i32,
    new_recipe: &NewRecipe,
) -> Result<Option<()>, StorageError> {
    let mut transaction = db_conn.begin().await?;

    let n_rows: u64 = query!(
        "
            UPDATE recipes SET
                name = $1,
                notes = $2,
                preparation_time_min = $3,
                cooking_time_min = $4,
                image = $5,
                instructions = $6,
                n_shares = $7,
                is_private = $8
            WHERE id = $9
        ",
        new_recipe.name,
        new_recipe.notes,
        new_recipe.preparation_time_min,
        new_recipe.cooking_time_min,
        new_recipe.image,
        &new_recipe.instructions,
        new_recipe.n_shares,
        new_recipe.is_private,
        id
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();

    if n_rows <= 0 {
        return Ok(None);
    }

    // Tags
    if !new_recipe.tags.is_empty() {
        query!(
            "
                INSERT INTO recipes_tags
                (tag_id, recipe_id)
                SELECT tag_id, $1 FROM UNNEST($2::int[]) as tag_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.tags
        )
        .execute(&mut *transaction)
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
        &new_recipe.tags
    )
    .execute(&mut *transaction)
    .await?;

    // Seasons
    if !new_recipe.seasons.is_empty() {
        query!(
            "
                INSERT INTO recipes_seasons
                (season_id, recipe_id)
                SELECT season_id, $1 FROM UNNEST($2::int[]) as season_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.seasons
        )
        .execute(&mut *transaction)
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
        &new_recipe.seasons
    )
    .execute(&mut *transaction)
    .await?;

    // Categories
    if !new_recipe.categories.is_empty() {
        query!(
            "
                INSERT INTO recipes_categories
                (category_id, recipe_id)
                SELECT category_id, $1 FROM UNNEST($2::int[]) as category_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.categories
        )
        .execute(&mut *transaction)
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
        &new_recipe.categories
    )
    .execute(&mut *transaction)
    .await?;

    // Diets
    if !new_recipe.diets.is_empty() {
        query!(
            "
                INSERT INTO recipes_diets
                (diet_id, recipe_id)
                SELECT diet_id, $1 FROM UNNEST($2::int[]) as diet_id
                ON CONFLICT DO NOTHING;
            ",
            id,
            &new_recipe.diets
        )
        .execute(&mut *transaction)
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
        &new_recipe.diets
    )
    .execute(&mut *transaction)
    .await?;

    // Ingredients
    let mut ingr_ids: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());
    let mut qtys: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());
    let mut unit_ids: Vec<_> = Vec::with_capacity(new_recipe.ingredients.len());

    new_recipe.ingredients.iter().for_each(|ref v| {
        ingr_ids.push(v.id);
        qtys.push(v.quantity);
        unit_ids.push(v.unit_id);
    });

    if !new_recipe.ingredients.is_empty() {
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
        .execute(&mut *transaction)
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
    .execute(&mut *transaction)
    .await?;
    transaction.commit().await?;

    Ok(Some(()))
}

pub async fn delete_recipe(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<()>, StorageError> {
    let n_rows: u64 = query!(
        "
            DELETE FROM recipes
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
