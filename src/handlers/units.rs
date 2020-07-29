use actix_web::{get, post, put, delete, web, Responder, http};
use crate::database::Pool;
use log::*;
use tokio_postgres::error::SqlState;

use crate::resources::unit;
use crate::utils::*;
use super::*;

static MAX_PER_REQUEST: i64 = 100;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_one)
       .service(delete_one)
    ;
}

#[get("/units")]
pub async fn get_all(params: web::Query<HttpParams>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let total_count: i64 = match get_total_count(&db_conn, "units").await {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let accept_range = format!("unit {}", MAX_PER_REQUEST);

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
    let units = match unit::get_many(&db_conn, range_first, range_last).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    let fetched_count = units.len() as i64;
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
       .body(format!("{}", serde_json::to_string_pretty(&units).unwrap()))
}

#[post("/units")]
pub async fn add_one(new_unit: web::Json<unit::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_unit);
    let new_id = match unit::add_one(&db_conn, &new_unit).await {
        Ok(v) => v,
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return web::HttpResponse::Conflict().finish();
        },
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        }
    };

    web::HttpResponse::Created()
        .set_header(http::header::LOCATION, format!("/{}", new_id))
        .finish()
}

#[get("/units/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let unit = match unit::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => { return web::HttpResponse::NotFound().finish(); },
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    };

    trace!("{}", serde_json::to_string_pretty(&unit).unwrap());
    web::HttpResponse::Ok().json(unit)
}

#[put("/units/{id}")]
pub async fn modify_one(id: web::Path<i32>, new_unit: web::Json<unit::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_unit);

    match unit::modify_one(&db_conn, id.into_inner(), &new_unit).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
            => return web::HttpResponse::Conflict().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    }
    web::HttpResponse::Ok().finish()
}

#[delete("/units/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    match unit::delete_one(&db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return web::HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return web::HttpResponse::InternalServerError().finish();
        },
    }
    web::HttpResponse::NoContent().finish()
}
