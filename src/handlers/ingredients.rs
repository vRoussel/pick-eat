use actix_web::{get, post, put, delete, web, Responder};
use log::*;
use tokio_postgres::{Client, error::SqlState};

use crate::resources::ingredient::{DBIngredient, NewIngredient};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_or_create_one)
       .service(delete_one)
    ;
}

#[get("/ingredients")]
pub async fn get_all() -> impl Responder {
    "Get all ingredients"
}

#[post("/ingredients")]
pub async fn add_one(new_ingredient: web::Json<NewIngredient>, db_conn: web::Data<Client>) -> impl Responder {
    trace!("{:#?}", new_ingredient);
    let insert_query = "\
        INSERT INTO ingredients (name, default_unit_id) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    let new_id = match db_conn.query(insert_query, &[&new_ingredient.name, &new_ingredient.default_unit_id])
        .await {
            Ok(rows) => rows[0].get::<&str,i32>("id").to_string(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                //TODO add location with URI
                => return web::HttpResponse::Conflict().finish(),
            Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
                => return web::HttpResponse::UnprocessableEntity().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
        };
    //TODO add location with URI
    web::HttpResponse::Created().finish()
}

#[get("/ingredients/{id}")]
pub async fn get_one(id: web::Path<i32>, db_conn: web::Data<Client>) -> impl Responder {
    let id = id.into_inner();
    let query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            u.id as default_unit_id, \
            u.full_name as default_unit_full_name, \
            u.short_name as default_unit_short_name \
        FROM
            ingredients as i \
            LEFT JOIN quantity_units as u \
            ON i.default_unit_id = u.id \
        WHERE
            i.id = $1 \
    ";

    let ingredient = match db_conn.query(query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => DBIngredient::from(&rows[0]),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Err(e) => {error!("{}", e); return web::HttpResponse::InternalServerError().finish()},
            _ => return web::HttpResponse::InternalServerError().finish(),
    };
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&ingredient).unwrap()))

}

#[put("/ingredients/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put ingredient {}", id)
}

#[delete("/ingredients/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete ingredient {}", id)
}
