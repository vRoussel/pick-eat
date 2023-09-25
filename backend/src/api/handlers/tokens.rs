use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::*;

use crate::{api::models, app::App};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_account_validation_token)
        .service(create_password_reset_token)
        .service(use_account_validation_token)
        .service(use_password_reset_token);
}

async fn create_token(
    app: web::Data<App>,
    token_req: web::Json<models::TokenNewRequestIn>,
    token_type: crate::models::TokenType,
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

    let t: crate::models::TokenNewRequest = token_req.into_inner().into();
    match app.generate_token(&t, token_type, &base_url).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            e.into()
        }
    }
}

#[post("/accounts/validation/tokenrequest")]
async fn create_account_validation_token(
    app: web::Data<App>,
    token_req: web::Json<models::TokenNewRequestIn>,
    req: HttpRequest,
) -> impl Responder {
    create_token(
        app,
        token_req,
        crate::models::TokenType::AccountValidation,
        req,
    )
    .await
}

#[post("/accounts/passwordreset/tokenrequest")]
async fn create_password_reset_token(
    app: web::Data<App>,
    token_req: web::Json<models::TokenNewRequestIn>,
    req: HttpRequest,
) -> impl Responder {
    create_token(app, token_req, crate::models::TokenType::PasswordReset, req).await
}

#[post("/accounts/validation")]
async fn use_account_validation_token(
    app: web::Data<App>,
    token_req: web::Json<models::AccountValidationTokenUseRequestIn>,
) -> impl Responder {
    let t: crate::models::AccountValidationTokenUseRequest = token_req.into_inner().into();
    match app.use_account_validation_token(&t).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            e.into()
        }
    }
}

#[post("/accounts/passwordreset")]
async fn use_password_reset_token(
    app: web::Data<App>,
    token_req: web::Json<models::PasswordResetTokenUseRequestIn>,
) -> impl Responder {
    let mut t: crate::models::PasswordResetTokenUseRequest = token_req.into_inner().into();
    match app.use_password_reset_token(&mut t).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            e.into()
        }
    }
}
