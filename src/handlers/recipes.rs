use actix_web::{get, post, put, delete, web, Responder};
use crate::database::Pool;
use log::*;
use tokio_postgres::{error::SqlState, types::ToSql};

use crate::resources::{
    category,
    tag,
    recipe,
    ingredient,
    ingredient::quantified as QIngredient
};

use crate::utils::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_or_create_one)
       .service(delete_one)
    ;
}

#[get("/recipes")]
pub async fn get_all() -> impl Responder {
    "Get all recipes"
}

#[post("/recipes")]
pub async fn add_one(new_recipe: web::Json<recipe::New>, db_pool: web::Data<Pool>) -> impl Responder {
    //TODO multiple inserts so we need a transaction
    let mut db_conn = db_pool.get().await.unwrap();
    let transaction = db_conn.transaction().await.expect("Unable to start db transaction");
    trace!("{:#?}", new_recipe);
    let recipe_query = "\
        INSERT INTO recipes \
        (name, description, preparation_time_min, cooking_time_min, image, instructions) \
        VALUES ($1, $2, $3, $4, $5, $6) \
        RETURNING id; \
    ";
    let new_id = match transaction.query(recipe_query,
        &[&new_recipe.name, &new_recipe.desc,
          &new_recipe.prep_time_min, &new_recipe.cook_time_min,
          &new_recipe.image, &new_recipe.instructions
        ]).await {
            Ok(rows) => rows[0].get::<&str,i32>("id"),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                //TODO add location with URI
                => return web::HttpResponse::Conflict().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
        };

    // Tags
    let values_query_params = gen_query_params(new_recipe.tag_ids.len(), 2);
    let tags_query = format!("\
        INSERT INTO recipes_tags \
        (tag_id, recipe_id) \
        VALUES {}; \
    ", values_query_params);

    let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
    for tag_id in &new_recipe.tag_ids {
        flat_values.extend_from_slice(&[tag_id, &new_id]);
    }

    match transaction.execute(tags_query.as_str(), &flat_values).await {
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
            => return web::HttpResponse::UnprocessableEntity().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
        _ => ()
    }

    // Categories
    let values_query_params = gen_query_params(new_recipe.category_ids.len(), 2);
    let categories_query = format!("\
        INSERT INTO recipes_categories \
        (category_id, recipe_id) \
        VALUES {}; \
    ", values_query_params);

    let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
    for category_id in &new_recipe.category_ids {
        flat_values.extend_from_slice(&[category_id, &new_id]);
    }

    match transaction.execute(categories_query.as_str(), &flat_values).await {
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
            => return web::HttpResponse::UnprocessableEntity().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
        _ => ()
    }

    // Ingredients
    let values_query_params = gen_query_params(new_recipe.q_ingredient_ids.len(), 4);
    let ingredients_query = format!("\
        INSERT INTO recipes_ingredients \
        (recipe_id, ingredient_id, quantity, unit_id) \
        VALUES {}; \
    ", values_query_params);

    let mut flat_values: Vec<&(dyn ToSql + Sync)> = Vec::new();
    for ingr in &new_recipe.q_ingredient_ids {
        flat_values.extend_from_slice(&[&new_id, &ingr.id, &ingr.quantity, &ingr.unit_id]);
    }

    match transaction.execute(ingredients_query.as_str(), &flat_values).await {
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
            => return web::HttpResponse::UnprocessableEntity().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
        _ => ()
    }

    //TODO add location with URI
    transaction.commit().await.expect("Error when commiting transaction");
    web::HttpResponse::Created().finish()
}

#[get("/recipes/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let recipe_query = "\
        SELECT \
            id, \
            name, \
            description, \
            preparation_time_min, \
            cooking_time_min, \
            image, \
            publication_date, \
            instructions \
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


    let mut recipe: recipe::FromDB = match db_conn.query(recipe_query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => (&rows[0]).into(),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Ok(_) => return web::HttpResponse::InternalServerError().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };

    let categories: Vec<category::FromDB> = match db_conn.query(categories_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| r.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.categories = categories;

    let tags: Vec<tag::FromDB> = match db_conn.query(tags_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| r.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.tags = tags;

    let ingredients: Vec<QIngredient::Full> = match db_conn.query(ingredients_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| r.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.q_ingredients = ingredients;

    //web::HttpResponse::Ok().body(format!("{}", serde_json::to_string(&recipe).unwrap()))
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&recipe).unwrap()))
}

#[put("/recipes/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put recipe {}", id)
}

#[delete("/recipes/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete recipe {}", id)
}
