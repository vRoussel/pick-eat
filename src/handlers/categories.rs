use actix_web::{get, post, put, delete, web, Responder};
use log::*;
use tokio_postgres::Client;

use crate::resources::category::{DBCategory, NewCategory, CategoryUpdate};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_or_create_one)
       .service(delete_one)
    ;
}

#[get("/categories")]
pub async fn get_all() -> impl Responder {
    "Get all categories"
}

#[post("/categories")]
pub async fn add_one() -> impl Responder {
    "Add new category"
}

#[get("/categories/{id}")]
pub async fn get_one(id: web::Path<i32>, db_conn: web::Data<Client>) -> impl Responder {
    let id = id.into_inner();
    let query = "\
        SELECT \
            id, \
            name \
        FROM categories \
        WHERE id = $1 \
    ";

    let category = match db_conn.query(query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => DBCategory::from(&rows[0]),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Ok(_) => return web::HttpResponse::InternalServerError().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&category).unwrap()))
}

#[put("/categories/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put category {}", id)
}

#[delete("/categories/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete category {}", id)
}
