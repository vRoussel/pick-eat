use crate::database::Pool;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;
use tokio_postgres::error::SqlState;

use crate::query_params::{Range, RangeError};
use crate::resources::{category, get_total_count};

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

#[get("/categories")]
pub async fn get_all(
    params: web::Query<GetQueryParams>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let total_count: i64 = match get_total_count(&db_conn, "categories").await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let accept_range = format!("category {}", MAX_PER_REQUEST.unwrap_or(0));

    if let Some(range) = &params.range {
        if let Err(e) = range.validate(MAX_PER_REQUEST, total_count) {
            let content_range = format!("{}-{}/{}", 0, 0, total_count);
            let mut ret = match e {
                RangeError::OutOfBounds => HttpResponse::NoContent(),
                RangeError::TooWide => HttpResponse::BadRequest(),
                RangeError::Invalid => HttpResponse::BadRequest(),
            };

            return ret
                .insert_header((http::header::CONTENT_RANGE, content_range))
                .insert_header((http::header::ACCEPT_RANGES, accept_range.clone()))
                .finish();
        }
    }

    let categories = match category::get_many(&db_conn, &params.range).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let fetched_count = categories.len() as i64;
    let mut ret;
    if fetched_count < total_count {
        ret = HttpResponse::PartialContent();
    } else {
        ret = HttpResponse::Ok();
    }

    let first_fetched = match &params.range {
        Some(r) => r.from,
        None => 1,
    };
    let last_fetched = first_fetched + fetched_count - 1;
    let content_range = format!("{}-{}/{}", first_fetched, last_fetched, total_count);

    trace!("{}", serde_json::to_string_pretty(&categories).unwrap());
    ret.insert_header((http::header::CONTENT_RANGE, content_range))
        .insert_header((http::header::ACCEPT_RANGES, accept_range))
        .json(categories)
}

#[post("/categories")]
pub async fn add_one(
    new_category: web::Json<category::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_category);
    let new_id = match category::add_one(&db_conn, &new_category).await {
        Ok(v) => v,
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return HttpResponse::Conflict().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[get("/categories/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let category = match category::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&category).unwrap());
    HttpResponse::Ok().json(category)
}

#[put("/categories/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_category: web::Json<category::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_category);

    match category::modify_one(&db_conn, id.into_inner(), &new_category).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return HttpResponse::Conflict().finish()
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[delete("/categories/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    match category::delete_one(&db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
