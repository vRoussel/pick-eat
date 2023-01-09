use super::db_error_to_http_response;
use actix_web::{delete, http, post, web, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::resources::account;
use crate::resources::account::AddAccountError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_one).service(delete_one);
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
            AddAccountError::DBError(Error::Database(db_error)) => {
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
