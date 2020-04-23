use actix_web::{get, post, put, delete, web, Responder};
use crate::resources::unit::Unit;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_or_create_one)
       .service(delete_one)
    ;
}

#[get("/units")]
pub async fn get_all() -> impl Responder {
    "Get all units"
}

#[post("/units")]
pub async fn add_one() -> impl Responder {
    "Add new unit"
}

#[get("/units/{id}")]
pub async fn get_one(id: web::Path<String>) -> impl Responder {
    format!("Get unit {}", id)
}

#[put("/units/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put unit {}", id)
}

#[delete("/units/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete unit {}", id)
}
