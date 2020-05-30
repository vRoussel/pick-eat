use actix_web::{get, post, put, delete, web, Responder};
use crate::database::Pool;
use log::*;
use tokio_postgres::error::SqlState;

use crate::resources::unit::{DBUnit, NewUnit};

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
pub async fn add_one(new_unit: web::Json<NewUnit>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_unit);
    let insert_query = "\
        INSERT INTO quantity_units (full_name, short_name) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    let new_id = match db_conn.query(insert_query, &[&new_unit.full_name, &new_unit.short_name])
        .await {
            Ok(rows) => rows[0].get::<&str,i32>("id").to_string(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                //TODO add location with URI
                => return web::HttpResponse::Conflict().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
        };
    //TODO add location with URI
    web::HttpResponse::Created().finish()
}

#[get("/units/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let query = "\
        SELECT \
            id, \
            full_name, \
            short_name \
        FROM quantity_units \
        WHERE id = $1 \
    ";

    let unit = match db_conn.query(query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => DBUnit::from(&rows[0]),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Ok(_) => return web::HttpResponse::InternalServerError().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&unit).unwrap()))

}

#[put("/units/{id}")]
pub async fn modify_or_create_one(id: web::Path<String>) -> impl Responder {
    format!("Put unit {}", id)
}

#[delete("/units/{id}")]
pub async fn delete_one(id: web::Path<String>) -> impl Responder {
    format!("Delete unit {}", id)
}
