use crate::resources::diet;
use actix_web::{get, web, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all).service(get_one);
}

#[get("/diets")]
pub async fn get_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let diets = match diet::get_all(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&diets).unwrap());
    HttpResponse::Ok().json(diets)
}

#[get("/diets/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let diet = match diet::get_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&diet).unwrap());
    HttpResponse::Ok().json(diet)
}
