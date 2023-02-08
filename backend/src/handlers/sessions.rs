use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::postgres::PgPool;

use crate::{handlers::User, resources::account};

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
    session: Session,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    let account_id =
        match account::check_credentials(&mut db_conn, &cred.email, &cred.password).await {
            Ok(account_id) => account_id,
            Err(_) => return HttpResponse::Unauthorized().finish(),
        };
    let account = match account::get_one(&mut db_conn, account_id).await {
        Ok(Some(account)) => account,
        _ => return HttpResponse::InternalServerError().finish(),
    };
    Identity::login(&request.extensions(), account_id.to_string()).unwrap();
    let session_user = User {
        id: account.id,
        is_admin: account.is_admin,
    };
    if session
        .insert(account.id.to_string(), session_user)
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[delete("/sessions/current")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent().finish()
}
