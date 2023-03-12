use super::{db_error_to_http_response, APIAnswer};
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;
use sqlx::{Error, PgConnection};

use crate::handlers::Admin;
use crate::resources::category;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(delete_one);
}

async fn validate_new_category(
    categ: &category::New,
    api_answer: &mut APIAnswer,
    db_conn: &mut PgConnection,
) {
    if let Ok(Some(_)) = category::get_one_by_name(db_conn, &categ.name).await {
        api_answer.add_field_error("name", "Une catégorie avec ce nom existe déjà");
    }
}

#[get("/categories")]
pub async fn get_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let categories = match category::get_all(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&categories).unwrap());
    HttpResponse::Ok().json(categories)
}

#[post("/categories")]
pub async fn add_one(
    new_category: web::Json<category::New>,
    db_pool: web::Data<PgPool>,
    _admin: Admin,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_category);

    let mut ret = APIAnswer::new();
    validate_new_category(&new_category, &mut ret, &mut db_conn).await;
    if !ret.is_ok() {
        return HttpResponse::BadRequest().json(ret);
    }

    let new_id = match category::add_one(&mut db_conn, &new_category).await {
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

#[get("/categories/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let category = match category::get_one(&mut db_conn, id.into_inner()).await {
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
    db_pool: web::Data<PgPool>,
    _admin: Admin,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    trace!("{:#?}", new_category);

    let mut ret = APIAnswer::new();
    validate_new_category(&new_category, &mut ret, &mut db_conn).await;
    if !ret.is_ok() {
        return HttpResponse::BadRequest().json(ret);
    }

    match category::modify_one(&mut db_conn, id.into_inner(), &new_category).await {
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

#[delete("/categories/{id}")]
pub async fn delete_one(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
    _admin: Admin,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match category::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
