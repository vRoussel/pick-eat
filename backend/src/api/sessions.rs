use actix_identity::Identity;
use actix_session::Session;
use actix_web::{delete, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use log::*;
use serde::Deserialize;


use crate::{
    api::{APIAnswer, APIError, User},
    app::App,
    models,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login).service(logout);
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialsIn {
    email: String,
    password: String,
}

impl From<CredentialsIn> for models::Credentials {
    fn from(c: CredentialsIn) -> Self {
        Self {
            email: c.email,
            password: models::Password::ClearText(c.password),
        }
    }
}

#[post("/sessions")]
pub async fn login(
    request: HttpRequest,
    cred: web::Json<CredentialsIn>,
    session: Session,
    app: web::Data<App>,
) -> impl Responder {
    let c: models::Credentials = cred.into_inner().into();
    let account = match app.get_account_from_credentials(&c).await {
        Ok(Some(account)) => account,
        Ok(None) => {
            let api_error = APIError {
                message: "Email ou mot de passe incorrect",
                field: None,
                code: Some("account_pending_validation"),
            };
            return HttpResponse::Unauthorized().json(APIAnswer::from(api_error));
        }
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    if !account.is_validated {
        let api_error = APIError {
            message: "Veuillez valider votre compte avec le lien reçu par mail",
            field: None,
            code: Some("account_pending_validation"),
        };
        return HttpResponse::Unauthorized().json(APIAnswer::from(api_error));
    }

    if let Err(e) = Identity::login(&request.extensions(), account.id.to_string()) {
        error!("{}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let session_user = User {
        id: account.id,
        is_admin: account.is_admin,
    };

    if let Err(e) = session.insert(account.id.to_string(), session_user) {
        error!("{}", e);
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}

#[delete("/sessions/current")]
pub async fn logout(user: Identity) -> impl Responder {
    user.logout();
    HttpResponse::NoContent().finish()
}
