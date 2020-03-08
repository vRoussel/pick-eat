use actix_web::{get, post, put, delete, web, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_or_create_one)
       .service(delete_one)
    ;
}

#[get("/tags")]
pub async fn get_all() -> impl Responder {
    "Get all tags"
}

#[post("/tags")]
pub async fn add_one() -> impl Responder {
    "Add new tag"
}

#[get("/tags/{id}")]
pub async fn get_one(id: web::Path<String>) -> impl Responder {
    format!("Get tag {}", id)
}

#[put("/tags/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put tag {}", id)
}

#[delete("/tags/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete tag {}", id)
}


