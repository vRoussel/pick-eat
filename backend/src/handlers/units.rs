use super::db_error_to_http_response;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::resources::unit;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(delete_one);
}

#[get("/units")]
pub async fn get_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let units = match unit::get_all(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&units).unwrap());
    HttpResponse::Ok().json(units)
}

#[post("/units")]
pub async fn add_one(new_unit: web::Json<unit::New>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_unit);
    let new_id = match unit::add_one(&mut db_conn, &new_unit).await {
        Ok(v) => v,
        Err(e) => match e {
            Error::Database(db_error) => {
                error!("{}", db_error);
                return db_error_to_http_response(&*db_error);
            }
            _ => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[get("/units/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let unit = match unit::get_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&unit).unwrap());
    HttpResponse::Ok().json(unit)
}

#[put("/units/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_unit: web::Json<unit::New>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_unit);

    match unit::modify_one(&mut db_conn, id.into_inner(), &new_unit).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => match e {
            Error::Database(db_error) => {
                error!("{}", db_error);
                return db_error_to_http_response(&*db_error);
            }
            _ => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
    HttpResponse::Ok().finish()
}

#[delete("/units/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match unit::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
