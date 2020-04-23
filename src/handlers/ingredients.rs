use actix_web::{get, post, put, delete, web, Responder};
use crate::resources::ingredient::Ingredient;

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
pub async fn add_one() -> impl Responder {
    "Add new ingredient"
}

#[get("/ingredients/{id}")]
pub async fn get_one(id: web::Path<String>) -> impl Responder {
    format!("Get ingredient {}", id)
}

#[put("/ingredients/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put ingredient {}", id)
}

#[delete("/ingredients/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete ingredient {}", id)
}
