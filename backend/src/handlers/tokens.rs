use actix_web::{post, web, HttpResponse, Responder};
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
    req: web::Json<token::New>,
) -> impl Responder {
    let mut transaction = db_pool.begin().await.unwrap();

    let token = match token::add_validation_token(&mut transaction, &req).await {
        Ok(t) => t,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if let Some(t) = token {
        if let Err(e) = email_sender
            .send_account_validation_email(&req.email, &t.token)
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
