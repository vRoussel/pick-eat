use super::db_error_to_http_response;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::resources::ingredient;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(delete_one);
}

#[get("/ingredients")]
pub async fn get_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let ingredients = match ingredient::get_all(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&ingredients).unwrap());
    HttpResponse::Ok().json(ingredients)
}

#[post("/ingredients")]
pub async fn add_one(
    new_ingredient: web::Json<ingredient::New>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_ingredient);
    let new_id = match ingredient::add_one(&mut db_conn, &new_ingredient).await {
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

#[get("/ingredients/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let ingredient = match ingredient::get_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&ingredient).unwrap());
    HttpResponse::Ok().json(ingredient)
}

#[put("/ingredients/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_ingredient: web::Json<ingredient::New>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_ingredient);

    match ingredient::modify_one(&mut db_conn, id.into_inner(), &new_ingredient).await {
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

#[delete("/ingredients/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match ingredient::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
