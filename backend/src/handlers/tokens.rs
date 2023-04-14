use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::*;
use sqlx::postgres::PgPool;

use crate::{email::EmailSender, resources::token};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_account_validation_token);
}

#[post("/accounts/validation/tokenrequest")]
pub async fn create_account_validation_token(
    db_pool: web::Data<PgPool>,
    email_sender: web::Data<EmailSender>,
    body: web::Json<token::New>,
    req: HttpRequest,
) -> impl Responder {
    let headers = req.headers();
    let host = match headers.get("Host") {
        Some(h) => h.to_str().unwrap(),
        None => return HttpResponse::BadRequest().finish(),
    };

    let scheme = match headers.get("X-Forwarded-Proto") {
        Some(h) => h.to_str().unwrap(),
        None => return HttpResponse::BadRequest().finish(),
    };

    let base_url = format!("{}://{}", scheme, host);

    let mut transaction = db_pool.begin().await.unwrap();

    let token = match token::add_validation_token(&mut transaction, &body).await {
        Ok(t) => t,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(t) = token {
        if let Err(e) = email_sender
            .send_account_validation_email(&body.email, &t.token, &base_url)
            .await
        {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }

    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok().finish()
}
