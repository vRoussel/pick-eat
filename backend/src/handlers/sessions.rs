use actix_identity::Identity;
use actix_web::{delete, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::resources::account;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout);
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Credentials {
    email: String,
    password: String,
}

#[post("/sessions")]
pub async fn login(
    request: HttpRequest,
    cred: web::Json<Credentials>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    match account::check_credentials(&mut db_conn, &cred.email, &cred.password).await {
        Ok(account_id) => Identity::login(&request.extensions(), account_id.to_string()).unwrap(),
        Err(_) => return HttpResponse::Unauthorized(),
    };
    HttpResponse::Ok()
}

#[delete("/sessions/current")]
pub async fn logout(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        user.logout();
        HttpResponse::NoContent().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}
