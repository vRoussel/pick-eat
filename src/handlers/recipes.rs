use actix_web::{get, post, put, delete, web, Responder};
use log::*;
use tokio_postgres::Client;

use crate::resources::{
    category::Category,
    ingredient::QuantifiedIngredient,
    tag::Tag,
    recipe::Recipe
};

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
pub async fn add_one() -> impl Responder {
    "Add new recipe"
}

#[get("/recipes/{id}")]
pub async fn get_one(id: web::Path<i32>, db_conn: web::Data<Client>) -> impl Responder {
    let id = id.into_inner();
    let recipe_query = "\
        SELECT \
            id, \
            name, \
            description, \
            (extract(epoch from preparation_time)/60)::integer as preparation_time_min, \
            (extract(epoch from cooking_time)/60)::integer as cooking_time_min, \
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
            u.short_name as unit_short_name, \
            u2.id as default_unit_id, \
            u2.full_name as default_unit_full_name, \
            u2.short_name as default_unit_short_name
        FROM \
            ingredients as i, \
            recipes_ingredients as ri, \
            quantity_units as u, \
            quantity_units as u2 \
        WHERE \
            i.id = ri.ingredient_id \
            AND ri.unit_id = u.id \
            AND i.default_unit_id = u2.id \
            AND ri.recipe_id = $1 \
    ";


    let mut recipe = match db_conn.query(recipe_query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => Recipe::from(&rows[0]),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Ok(_) => return web::HttpResponse::InternalServerError().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };

    let categories: Vec<_> = match db_conn.query(categories_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| Category::from(r)).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.categories = categories;

    let tags: Vec<_> = match db_conn.query(tags_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| Tag::from(r)).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.tags = tags;

    let ingredients: Vec<_> = match db_conn.query(ingredients_query, &[&id])
        .await {
            Ok(rows) => rows.iter().map(|r| QuantifiedIngredient::from(r)).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    recipe.ingredients = ingredients;

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
