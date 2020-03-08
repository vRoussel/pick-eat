use actix_web::{get, post, put, delete, web, Responder};

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
pub async fn get_one(id: web::Path<String>) -> impl Responder {
    format!("Get category {}", id)
}

#[put("/categories/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put category {}", id)
}

#[delete("/categories/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete category {}", id)
}


