use crate::database::Pool;
use actix_web::{delete, get, http, post, put, web, Responder};
use log::*;
use serde::Deserialize;
use tokio_postgres::error::SqlState;

use crate::query_params::{Range, RangeError};
use crate::resources::{get_total_count, ingredient};

static MAX_PER_REQUEST: Option<i64> = None;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(delete_one);
}

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    range: Option<Range>,
}

#[get("/ingredients")]
pub async fn get_all(
    params: web::Query<GetQueryParams>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let total_count: i64 = match get_total_count(&db_conn, "ingredients").await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    let accept_range = format!("ingredient {}", MAX_PER_REQUEST.unwrap_or(0));

    if let Some(range) = &params.range {
        if let Err(e) = range.validate(MAX_PER_REQUEST, total_count) {
            let content_range = format!("{}-{}/{}", 0, 0, total_count);
            let mut ret = match e {
                RangeError::OutOfBounds => web::HttpResponse::NoContent(),
                RangeError::TooWide => web::HttpResponse::BadRequest(),
                RangeError::Invalid => web::HttpResponse::BadRequest(),
            };

            return ret
                .set_header(http::header::CONTENT_RANGE, content_range)
                .set_header(http::header::ACCEPT_RANGES, accept_range.clone())
                .finish();
        }
    }

    let ingredients = match ingredient::get_many(&db_conn, &params.range).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    let fetched_count = ingredients.len() as i64;
    let mut ret;
    if fetched_count < total_count {
        ret = web::HttpResponse::PartialContent();
    } else {
        ret = web::HttpResponse::Ok();
    }

    let first_fetched = match &params.range {
        Some(r) => r.from,
        None => 1,
    };
    let last_fetched = first_fetched + fetched_count - 1;
    let content_range = format!("{}-{}/{}", first_fetched, last_fetched, total_count);

    trace!("{}", serde_json::to_string_pretty(&ingredients).unwrap());
    ret.set_header(http::header::CONTENT_RANGE, content_range)
        .set_header(http::header::ACCEPT_RANGES, accept_range)
        .json(ingredients)
}

#[post("/ingredients")]
pub async fn add_one(
    new_ingredient: web::Json<ingredient::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_ingredient);
    let new_id = match ingredient::add_one(&db_conn, &new_ingredient).await {
        Ok(v) => v,
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return web::HttpResponse::Conflict().finish();
        }
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    web::HttpResponse::Created()
        .set_header(http::header::LOCATION, format!("/{}", new_id))
        .finish()
}

#[get("/ingredients/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let ingredient = match ingredient::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return web::HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&ingredient).unwrap());
    web::HttpResponse::Ok().json(ingredient)
}

#[put("/ingredients/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_ingredient: web::Json<ingredient::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_ingredient);

    match ingredient::modify_one(&db_conn, id.into_inner(), &new_ingredient).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return web::HttpResponse::Conflict().finish()
        }
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    }
    web::HttpResponse::Ok().finish()
}

#[delete("/ingredients/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    match ingredient::delete_one(&db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    }
    web::HttpResponse::NoContent().finish()
}
