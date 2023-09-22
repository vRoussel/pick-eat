use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use log::*;
use serde::Deserialize;

use crate::{
    app::App,
    models::{
        self, InvalidAccountValidationTokenUseRequest, InvalidPasswordResetTokenUseRequest,
        InvalidTokenNewRequest,
    },
};

use super::APIError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_account_validation_token)
        .service(create_password_reset_token)
        .service(use_account_validation_token);
}

impl From<InvalidTokenNewRequest> for Vec<APIError> {
    fn from(value: InvalidTokenNewRequest) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.email {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez saisir une adresse mail",
                        field: Some("email"),
                        code: None,
                    });
                }
                Kind::BadFormat => {
                    ret.push(APIError {
                        message: "Cette adresse mail est invalide",
                        field: Some("email"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}

impl From<InvalidAccountValidationTokenUseRequest> for Vec<APIError> {
    fn from(value: InvalidAccountValidationTokenUseRequest) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.token {
            match v {
                Kind::Expired | Kind::NotFound => {
                    ret.push(APIError {
                        message: "Votre lien de validation est invalide ou expiré, veuillez en générer un nouveau en essayant de vous connecter",
                        field: None,
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}

impl From<InvalidPasswordResetTokenUseRequest> for Vec<APIError> {
    fn from(value: InvalidPasswordResetTokenUseRequest) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.token {
            match v {
                Kind::Expired | Kind::NotFound => {
                    ret.push(APIError {
                        message: "Votre lien de validation est invalide ou expiré, veuillez en générer un nouveau en essayant de vous connecter",
                        field: None,
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.new_password {
            match v {
                Kind::TooShort => Some(APIError {
                    message: "Le mot de passe doit contenir au moins 8 caractères",
                    field: Some("new_password"),
                    code: None,
                }),
                _ => {
                    warn!(
                        "{:?} error received for password_reset_token_use's new_password, this should not happen",
                        v
                    );
                    None
                }
            };
        };
        ret
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TokenNewRequestIn {
    email: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AccountValidationTokenUseRequestIn {
    token: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PasswordResetTokenUseRequestIn {
    token: String,
    new_password: String,
}

impl From<TokenNewRequestIn> for models::TokenNewRequest {
    fn from(t: TokenNewRequestIn) -> Self {
        Self { email: t.email }
    }
}

impl From<AccountValidationTokenUseRequestIn> for models::AccountValidationTokenUseRequest {
    fn from(t: AccountValidationTokenUseRequestIn) -> Self {
        Self { token: t.token }
    }
}

impl From<PasswordResetTokenUseRequestIn> for models::PasswordResetTokenUseRequest {
    fn from(t: PasswordResetTokenUseRequestIn) -> Self {
        Self {
            token: t.token,
            new_password: models::Password::ClearText(t.new_password),
        }
    }
}

async fn create_token(
    app: web::Data<App>,
    token_req: web::Json<TokenNewRequestIn>,
    token_type: models::TokenType,
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

    let t: models::TokenNewRequest = token_req.into_inner().into();
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
    token_req: web::Json<TokenNewRequestIn>,
    req: HttpRequest,
) -> impl Responder {
    create_token(app, token_req, models::TokenType::AccountValidation, req).await
}

#[post("/accounts/passwordreset/tokenrequest")]
async fn create_password_reset_token(
    app: web::Data<App>,
    token_req: web::Json<TokenNewRequestIn>,
    req: HttpRequest,
) -> impl Responder {
    create_token(app, token_req, models::TokenType::PasswordReset, req).await
}

#[post("/accounts/validation")]
async fn use_account_validation_token(
    app: web::Data<App>,
    token_req: web::Json<AccountValidationTokenUseRequestIn>,
) -> impl Responder {
    let t: models::AccountValidationTokenUseRequest = token_req.into_inner().into();
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
    token_req: web::Json<PasswordResetTokenUseRequestIn>,
) -> impl Responder {
    let mut t: models::PasswordResetTokenUseRequest = token_req.into_inner().into();
    match app.use_password_reset_token(&mut t).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            error!("{:?}", e);
            e.into()
        }
    }
}
