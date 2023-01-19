use super::db_error_to_http_response;
use actix_identity::Identity;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::resources::account;
use crate::resources::account::InsertAccountError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_one)
        .service(delete_one)
        .service(delete_me)
        .service(get_current)
        .service(get_all)
        .service(modify_current)
        .service(add_fav_recipe)
        .service(remove_fav_recipe);
}

#[post("/accounts")]
pub async fn add_one(
    account: web::Json<account::New>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    let new_id = match account::add_one(&mut db_conn, &account).await {
        Ok(v) => v,
        Err(e) => match e {
            InsertAccountError::DBError(Error::Database(db_error)) => {
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

#[delete("/accounts/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match account::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}

#[delete("/accounts/me")]
pub async fn delete_me(user: Identity, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    let user_id: i32 = match user.id().map(|id_str| id_str.parse()) {
        Ok(Ok(id)) => id,
        Ok(Err(e)) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match account::delete_one(&mut db_conn, user_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}

#[get("/accounts/me")]
pub async fn get_current(user: Identity, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    let user_id: i32 = match user.id().map(|id_str| id_str.parse()) {
        Ok(Ok(id)) => id,
        Ok(Err(e)) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let account = match account::get_one(&mut db_conn, user_id).await {
        Ok(Some(v)) => v,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(account)
}

#[derive(Debug, Deserialize)]
pub struct GetAllQueryParams {
    withrecipes: bool,
}

#[get("/accounts")]
pub async fn get_all(
    params: web::Query<GetAllQueryParams>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    if !params.withrecipes {
        return HttpResponse::BadRequest().finish();
    }

    let accounts = match account::get_all_with_recipes(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&accounts).unwrap());
    HttpResponse::Ok().json(accounts)
}

#[put("/accounts/me")]
pub async fn modify_current(
    account: web::Json<account::Update>,
    user: Identity,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let user_id: i32 = match user.id().map(|id_str| id_str.parse()) {
        Ok(Ok(id)) => id,
        Ok(Err(e)) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Err(e) = account::check_password(&mut db_conn, user_id, &account.old_password).await {
        return HttpResponse::Unauthorized().finish();
    };

    match account::modify_one(&mut db_conn, user_id, &account).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => match e {
            InsertAccountError::DBError(Error::Database(db_error)) => {
                error!("{}", db_error);
                return db_error_to_http_response(&*db_error);
            }
            _ => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Ok().finish()
}

#[put("/accounts/me/favorites/{recipe_id}")]
pub async fn add_fav_recipe(
    user: Identity,
    recipe_id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let user_id: i32 = match user.id().map(|id_str| id_str.parse()) {
        Ok(Ok(id)) => id,
        Ok(Err(e)) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match account::add_fav_recipe(&mut db_conn, user_id, *recipe_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[delete("/accounts/me/favorites/{recipe_id}")]
pub async fn remove_fav_recipe(
    user: Identity,
    recipe_id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let user_id: i32 = match user.id().map(|id_str| id_str.parse()) {
        Ok(Ok(id)) => id,
        Ok(Err(e)) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    match account::remove_fav_recipe(&mut db_conn, user_id, *recipe_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}
