use actix_web::{get, post, put, delete, web, Responder};

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
pub async fn get_one(id: web::Path<String>) -> impl Responder {
    format!("Get recipe {}", id)
}

#[put("/recipes/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put recipe {}", id)
}

#[delete("/recipes/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete recipe {}", id)
}


