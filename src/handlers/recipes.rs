use std::collections::HashMap;

use actix_web::{get, post, put, delete, web, Responder, http};
use crate::database::Pool;
use log::*;
use tokio_postgres::{error::SqlState, types::ToSql};

use serde::{Deserialize, Serialize};

use crate::resources::{
    category,
    tag,
    recipe,
    ingredient,
    ingredient::quantified as QIngredient
};
use crate::utils::*;
use super::*;

static MAX_PER_REQUEST: i64 = 50;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_one)
       .service(delete_one)
    ;
}

#[get("/recipes")]
pub async fn get_all(params: web::Query<HttpParams>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let total_count: i64 = match get_total_count(&db_conn, "recipes").await {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let accept_range = format!("recipe {}", MAX_PER_REQUEST);

    let range = params.range();
    if let Err(e) = check_range(range, MAX_PER_REQUEST, total_count) {
        let content_range = format!("{}-{}/{}", 0, 0, total_count);
        let mut ret = match e {
            RangeError::OutOfBounds => web::HttpResponse::NoContent(),
            RangeError::TooWide => web::HttpResponse::BadRequest(),
            RangeError::Invalid => web::HttpResponse::BadRequest(),
        };

        return ret.set_header(http::header::CONTENT_RANGE, content_range)
            .set_header(http::header::ACCEPT_RANGES, accept_range.clone())
            .finish()
    }

    let (range_first, range_last) = range;
    let categories = match recipe::get_many(&db_conn, range_first, range_last).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    let fetched_count = categories.len() as i64;
    let mut ret;
    if fetched_count < total_count {
        ret = web::HttpResponse::PartialContent();
    } else {
        ret = web::HttpResponse::Ok();
    }

    let first_fetched = range_first;
    let last_fetched = first_fetched + fetched_count - 1;
    let content_range = format!("{}-{}/{}", first_fetched, last_fetched, total_count);

    ret.set_header(http::header::CONTENT_RANGE, content_range)
       .set_header(http::header::ACCEPT_RANGES, accept_range)
       .body(format!("{}", serde_json::to_string_pretty(&categories).unwrap()))
}

#[post("/recipes")]
pub async fn add_one(new_recipe: web::Json<recipe::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let mut db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_recipe);
    let new_id = match recipe::add_one(&mut db_conn, &new_recipe).await {
        Ok(v) => v,
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
            => return web::HttpResponse::Conflict().finish(),
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
            => return web::HttpResponse::UnprocessableEntity().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    web::HttpResponse::Created()
        .set_header(http::header::LOCATION, format!("/{}", new_id))
        .finish()
}

#[get("/recipes/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let recipe = match recipe::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => { return web::HttpResponse::NotFound().finish(); },
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    };

    trace!("{}", serde_json::to_string_pretty(&recipe).unwrap());
    web::HttpResponse::Ok().json(recipe)
}

#[put("/recipes/{id}")]
pub async fn modify_one(id: web::Path<i32>, new_recipe: web::Json<recipe::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let mut db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_recipe);

    match recipe::modify_one(&mut db_conn, id.into_inner(), &new_recipe).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
            => return web::HttpResponse::Conflict().finish(),
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
                => return web::HttpResponse::UnprocessableEntity().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    }
    web::HttpResponse::Ok().finish()
}

#[delete("/recipes/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    match recipe::delete_one(&db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    }
    web::HttpResponse::NoContent().finish()
}
