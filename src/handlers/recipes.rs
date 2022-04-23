use actix_web::{delete, get, http, patch, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;
use tokio_postgres::error::SqlState;

use crate::database::Pool;
use crate::query_params::{Range, RangeError};
use crate::resources::{get_total_count, recipe};

static MAX_PER_REQUEST: Option<i64> = Some(50);

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(patch_one)
        .service(delete_one);
}

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    range: Range,
    search: Option<String>,
}

#[get("/recipes")]
pub async fn get_all(
    params: web::Query<GetQueryParams>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let accept_range = format!("recipe {}", MAX_PER_REQUEST.unwrap_or(0));

    if let Err(e) = params.range.validate(MAX_PER_REQUEST) {
        let content_range = format!("{}-{}/{}", 0, 0, "*");
        let mut ret = match e {
            RangeError::OutOfBounds => HttpResponse::NoContent(),
            RangeError::TooWide => HttpResponse::BadRequest(),
            RangeError::Invalid => HttpResponse::BadRequest(),
        };

        return ret
            .insert_header((http::header::CONTENT_RANGE, content_range))
            .insert_header((http::header::ACCEPT_RANGES, accept_range))
            .finish();
    }

    let (recipes, total_count) =
        match recipe::get_many(&db_conn, &params.range, &params.search).await {
            Ok(v) => v,
            Err(e) => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        };

    let fetched_count = recipes.len() as i64;
    let mut ret;
    let (first_fetched, last_fetched);

    if fetched_count == 0 {
        first_fetched = 0;
        last_fetched = 0;
    } else {
        first_fetched = params.range.from;
        last_fetched = first_fetched + fetched_count - 1;
    }

    if fetched_count == 0 {
        ret = HttpResponse::NoContent();
    } else if fetched_count < total_count {
        ret = HttpResponse::PartialContent();
    } else {
        ret = HttpResponse::Ok();
    }
    let content_range = format!("{}-{}/{}", first_fetched, last_fetched, total_count);

    trace!("{}", serde_json::to_string_pretty(&recipes).unwrap());
    ret.insert_header((http::header::CONTENT_RANGE, content_range))
        .insert_header((http::header::ACCEPT_RANGES, accept_range))
        .json(recipes)
}

#[post("/recipes")]
pub async fn add_one(
    new_recipe: web::Json<recipe::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_recipe);
    let new_id = match recipe::add_one(&mut db_conn, &new_recipe).await {
        Ok(v) => v,
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return HttpResponse::Conflict().finish()
        }
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION) => {
            return HttpResponse::UnprocessableEntity().finish()
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

#[get("/recipes/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let recipe = match recipe::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&recipe).unwrap());
    HttpResponse::Ok().json(recipe)
}

#[put("/recipes/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_recipe: web::Json<recipe::New>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_recipe);

    match recipe::modify_one(&mut db_conn, id.into_inner(), &new_recipe).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION) => {
            return HttpResponse::Conflict().finish()
        }
        Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION) => {
            return HttpResponse::UnprocessableEntity().finish()
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[patch("/recipes/{id}")]
pub async fn patch_one(
    id: web::Path<i32>,
    patched_recipe: web::Json<recipe::Patched>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let mut db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", patched_recipe);

    match recipe::patch_one(&mut db_conn, id.into_inner(), &patched_recipe).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[delete("/recipes/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    match recipe::delete_one(&db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
