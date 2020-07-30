use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio_postgres::{Client, error::Error, types::ToSql};
use super::category;
use super::tag;
use super::ingredient;
use super::ingredient::quantified as QIngredient;
use crate::utils::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) q_ingredients: Vec<QIngredient::Full>,
    pub(crate) categories: Vec<category::FromDB>,
    pub(crate) tags: Vec<tag::FromDB>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: Vec<u8>,
    pub(crate) publish_date: time::Date,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) q_ingredient_ids: Vec<QIngredient::Ref>,
    pub(crate) category_ids: Vec<i32>,
    pub(crate) tag_ids: Vec<i32>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: Vec<u8>,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16
}


impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            name: row.get("name"),
            desc: row.get("description"),
            q_ingredients: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions"),
            n_shares: row.get("n_shares")
        }
    }
}

pub async fn get_many(db_conn: &Client, min: i64, max: i64) -> Result<Vec<FromDB>, Error> {
    let recipes_query = "\
        SELECT \
            id, \
            name, \
            description, \
            preparation_time_min, \
            cooking_time_min, \
            image, \
            publication_date, \
            instructions, \
            n_shares \
        FROM recipes \
        ORDER BY name
        OFFSET $1
        LIMIT $2
    ";

    let mut recipes: Vec<FromDB> = db_conn.query(recipes_query, &[&(min-1), &(max-min+1)])
        .await
        .map(|rows|rows.iter().map(|r| r.into()).collect())?;


    let mut recipe_id_to_idx = HashMap::new();
    for (i,r) in recipes.iter().enumerate() {
        recipe_id_to_idx.insert(r.id, i);
    }
    let recipe_ids = recipe_id_to_idx.keys()
      .map(|s| s as &(dyn ToSql + Sync))
      .collect::<Vec<_>>();

    let query_params = gen_sql_query_params(recipe_ids.len(), 1);
    let categories_query = format!("\
        SELECT \
            rc.recipe_id, \
            c.id, \
            c.name \
        FROM \
            categories as c, \
            recipes_categories as rc \
        WHERE \
            c.id = rc.category_id \
            AND rc.recipe_id IN ({}) \
    ", query_params);

    db_conn.query(categories_query.as_str(), &recipe_ids)
        .await
        .map(|rows| {
            for r in &rows {
                let id: i32 = r.get("recipe_id");
                let idx  = recipe_id_to_idx.get(&id).unwrap();
                let recipe = recipes.get_mut(*idx).unwrap();
                recipe.categories.push(r.into());
            }
        })?;

    let tags_query = format!("\
        SELECT \
            rt.recipe_id, \
            t.id, \
            t.name \
        FROM \
            tags as t, \
            recipes_tags as rt \
        WHERE \
            t.id = rt.tag_id \
            AND rt.recipe_id IN ({}) \
    ", query_params);

    db_conn.query(tags_query.as_str(), &recipe_ids)
        .await
        .map(|rows| {
            for r in &rows {
                let id: i32 = r.get("recipe_id");
                let idx  = recipe_id_to_idx.get(&id).unwrap();
                let recipe = recipes.get_mut(*idx).unwrap();
                recipe.tags.push(r.into());
            }
        })?;


    let ingredients_query = format!("\
        SELECT \
            ri.recipe_id, \
            i.id as id, \
            i.name as name, \
            ri.quantity as quantity, \
            u.id as unit_id, \
            u.full_name as unit_full_name, \
            u.short_name as unit_short_name \
        FROM \
            ingredients as i, \
            recipes_ingredients as ri \
            LEFT JOIN units as u \
            ON ri.unit_id = u.id \
        WHERE \
            i.id = ri.ingredient_id \
            AND ri.recipe_id IN ({}) \
    ", query_params);

    db_conn.query(ingredients_query.as_str(), &recipe_ids)
        .await
        .map(|rows| {
            for r in &rows {
                let id: i32 = r.get("recipe_id");
                let idx  = recipe_id_to_idx.get(&id).unwrap();
                let recipe = recipes.get_mut(*idx).unwrap();
                recipe.q_ingredients.push(r.into());
            }
        })?;

    Ok(recipes)
}

pub async fn add_one(db_conn: &mut Client, new_recipe: &New) -> Result<i32, Error> {
    let transaction = db_conn.transaction().await.expect("Unable to start db transaction");
    let recipe_query = "\
        INSERT INTO recipes \
        (name, description, preparation_time_min, cooking_time_min, image, instructions, n_shares) \
        VALUES ($1, $2, $3, $4, $5, $6, $7) \
        RETURNING id; \
    ";

    let new_id: i32 = transaction.query(recipe_query,
        &[&new_recipe.name, &new_recipe.desc,
          &new_recipe.prep_time_min, &new_recipe.cook_time_min,
          &new_recipe.image, &new_recipe.instructions,
          &new_recipe.n_shares
        ]).await
          .map(|rows| rows[0].get(0))?;

    // Tags
    if !new_recipe.tag_ids.is_empty() {
        let values_query_params = gen_sql_query_params(new_recipe.tag_ids.len(), 2);
        let tags_query = format!("\
            INSERT INTO recipes_tags \
            (tag_id, recipe_id) \
            VALUES {}; \
        ", values_query_params);

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for tag_id in &new_recipe.tag_ids {
            flat_values.extend_from_slice(&[tag_id, &new_id]);
        }

        transaction.execute(tags_query.as_str(), &flat_values).await?;
    }

    // Categories
    if !new_recipe.category_ids.is_empty() {
        let values_query_params = gen_sql_query_params(new_recipe.category_ids.len(), 2);
        let categories_query = format!("\
            INSERT INTO recipes_categories \
            (category_id, recipe_id) \
            VALUES {}; \
        ", values_query_params);

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for category_id in &new_recipe.category_ids {
            flat_values.extend_from_slice(&[category_id, &new_id]);
        }

        transaction.execute(categories_query.as_str(), &flat_values).await?;
    }
    //
    // Ingredients
    if !new_recipe.q_ingredient_ids.is_empty() {
        let values_query_params = gen_sql_query_params(new_recipe.q_ingredient_ids.len(), 4);
        let ingredients_query = format!("\
            INSERT INTO recipes_ingredients \
            (recipe_id, ingredient_id, quantity, unit_id) \
            VALUES {}; \
        ", values_query_params);

        let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for ingr in &new_recipe.q_ingredient_ids {
            flat_values.extend_from_slice(&[&new_id, &ingr.id, &ingr.quantity, &ingr.unit_id]);
        }

        transaction.execute(ingredients_query.as_str(), &flat_values).await?;
    }

    transaction.commit().await.expect("Error when commiting transaction");
    Ok(new_id)
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let recipe_query = "\
        SELECT \
            id, \
            name, \
            description, \
            preparation_time_min, \
            cooking_time_min, \
            image, \
            publication_date, \
            instructions, \
            n_shares \
        FROM recipes \
        WHERE id = $1 \
    ";

    let categories_query = "\
        SELECT \
            c.id, \
            c.name \
        FROM \
            categories as c, \
            recipes_categories as rc \
        WHERE \
            c.id = rc.category_id \
            AND rc.recipe_id = $1 \
    ";

    let tags_query = "\
        SELECT \
            t.id, \
            t.name \
        FROM \
            tags as t, \
            recipes_tags as rt \
        WHERE \
            t.id = rt.tag_id \
            AND rt.recipe_id = $1 \
    ";

    let ingredients_query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            ri.quantity as quantity, \
            u.id as unit_id, \
            u.full_name as unit_full_name, \
            u.short_name as unit_short_name \
        FROM \
            ingredients as i, \
            recipes_ingredients as ri \
            LEFT JOIN units as u \
            ON ri.unit_id = u.id \
        WHERE \
            i.id = ri.ingredient_id \
            AND ri.recipe_id = $1 \
    ";

    let mut maybe_recipe: Option<FromDB> = db_conn.query_opt(recipe_query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))?;

    if let Some(ref mut recipe) = maybe_recipe {
        let categories: Vec<category::FromDB> = db_conn.query(categories_query, &[&id])
            .await
            .map(|rows|rows.iter().map(|r| r.into()).collect())?;

        let tags: Vec<tag::FromDB> = db_conn.query(tags_query, &[&id])
            .await
            .map(|rows|rows.iter().map(|r| r.into()).collect())?;

        let ingredients: Vec<QIngredient::Full> = db_conn.query(ingredients_query, &[&id])
            .await
            .map(|rows|rows.iter().map(|r| r.into()).collect())?;

        recipe.categories = categories;
        recipe.tags = tags;
        recipe.q_ingredients = ingredients;
    }

    Ok(maybe_recipe)
}

pub async fn modify_one(db_conn: &mut Client, id: i32, new_recipe: &New) -> Result<Option<()>, Error> {
    let transaction = db_conn.transaction().await.expect("Unable to start db transaction");

    let recipe_query = "\
        UPDATE recipes SET \
            name = $1, \
            description = $2, \
            preparation_time_min = $3, \
            cooking_time_min = $4, \
            image = $5, \
            instructions = $6, \
            n_shares = $7 \
        WHERE id = $8 \
        RETURNING id; \
    ";
    transaction.query_opt(recipe_query,
        &[&new_recipe.name, &new_recipe.desc,
          &new_recipe.prep_time_min, &new_recipe.cook_time_min,
          &new_recipe.image, &new_recipe.instructions,
          &new_recipe.n_shares, &id
         ]).await
      .map(|opt| opt.map(|_| ()))?; // OK(Some(row)) => Ok(Some(()))

    //TODO this is ugly and should be more simple

    // Tags
    if new_recipe.tag_ids.is_empty() {
        let remove_tags_query = "\
            DELETE FROM recipes_tags \
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_tags_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.tag_ids.len(), 2);
        let insert_tags_query = format!("\
            INSERT INTO recipes_tags \
            (tag_id, recipe_id) \
            VALUES {} \
            ON CONFLICT DO NOTHING;
        ", query_params);

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for tag_id in &new_recipe.tag_ids {
            query_args.extend_from_slice(&[tag_id, &id]);
        }
        transaction.execute(insert_tags_query.as_str(), &query_args).await?;

        let query_params = gen_sql_query_params_from(new_recipe.tag_ids.len(), 1, 2);
        let remove_tags_query = format!("\
            DELETE FROM recipes_tags \
            WHERE \
                recipe_id = $1 \
                AND tag_id NOT IN ({});
        ", query_params);
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for tag_id in &new_recipe.tag_ids {
            query_args.push(tag_id);
        }
        transaction.execute(remove_tags_query.as_str(), &query_args).await?;
    }

    // Categories
    if new_recipe.category_ids.is_empty() {
        let remove_categories_query = "\
            DELETE FROM recipes_categories \
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_categories_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.category_ids.len(), 2);
        let insert_categories_query = format!("\
            INSERT INTO recipes_categories \
            (category_id, recipe_id) \
            VALUES {} \
            ON CONFLICT DO NOTHING;
        ", query_params);

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for category_id in &new_recipe.category_ids {
            query_args.extend_from_slice(&[category_id, &id]);
        }
        transaction.execute(insert_categories_query.as_str(), &query_args).await?;

        let query_params = gen_sql_query_params_from(new_recipe.category_ids.len(), 1, 2);
        let remove_categories_query = format!("\
            DELETE FROM recipes_categories \
            WHERE \
                recipe_id = $1 \
                AND category_id NOT IN ({});
        ", query_params);
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for category_id in &new_recipe.category_ids {
            query_args.push(category_id);
        }
        transaction.execute(remove_categories_query.as_str(), &query_args).await?;
    }

    // Ingredients
    if new_recipe.q_ingredient_ids.is_empty() {
        let remove_ingredients_query = "\
            DELETE FROM recipes_ingredients \
            WHERE recipe_id = $1;
        ";
        transaction.execute(remove_ingredients_query, &[&id]).await?;
    } else {
        let query_params = gen_sql_query_params(new_recipe.q_ingredient_ids.len(), 4);
        let insert_ingredients_query = format!("\
            INSERT INTO recipes_ingredients \
            (recipe_id, ingredient_id, quantity, unit_id) \
            VALUES {} \
            ON CONFLICT DO NOTHING;
        ", query_params);

        let mut query_args: Vec<&(dyn ToSql + Sync)> = Vec::new();
        for ingr in &new_recipe.q_ingredient_ids {
            query_args.extend_from_slice(&[&id, &ingr.id, &ingr.quantity, &ingr.unit_id]);
        }
        transaction.execute(insert_ingredients_query.as_str(), &query_args).await?;

        let query_params = gen_sql_query_params_from(new_recipe.q_ingredient_ids.len(), 1, 2);
        let remove_ingredients_query = format!("\
            DELETE FROM recipes_ingredients \
            WHERE \
                recipe_id = $1 \
                AND ingredient_id NOT IN ({});
        ", query_params);
        let mut query_args: Vec<&(dyn ToSql + Sync)> = vec![&id];
        for ingr in &new_recipe.q_ingredient_ids {
            query_args.push(&ingr.id);
        }
        transaction.execute(remove_ingredients_query.as_str(), &query_args).await?;
    }

    transaction.commit().await.expect("Error when commiting transaction");
    Ok(Some(()))
}


pub async fn delete_one(db_conn: &Client, id: i32) -> Result<Option<()>, Error> {
    let delete_query = "\
        DELETE FROM recipes \
        WHERE id = $1 \
        RETURNING id;
    ";
    db_conn.query_opt(delete_query, &[&id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}
