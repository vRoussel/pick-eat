pub mod accounts;
pub mod categories;
pub mod diets;
pub mod ingredients;
pub mod recipes;
pub mod seasons;
pub mod sessions;
pub mod tags;
pub mod tokens;
pub mod units;

use std::{future::Future, pin::Pin, sync::RwLock};

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorUnauthorized},
    Error, FromRequest, HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::error::DatabaseError;

pub fn db_error_to_http_response(db_error: &dyn DatabaseError) -> HttpResponse {
    match db_error.code().as_deref() {
        Some("23505") => HttpResponse::Conflict().finish(),
        Some("23503") => HttpResponse::UnprocessableEntity().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct User {
    id: i32,
    is_admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Admin(User);

impl FromRequest for User {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<User, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = Identity::from_request(req, pl);
        let fut2 = Session::from_request(req, pl);
        Box::pin(async move {
            if let Ok(user_id) = fut.await?.id() {
                if let Ok(Some(user)) = fut2.await?.get(&user_id) {
                    return Ok(user);
                }
            };
            Err(ErrorUnauthorized("unauthorized"))
        })
    }
}

impl FromRequest for Admin {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Admin, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let fut = User::from_request(req, pl);
        Box::pin(async move {
            if let Ok(user) = fut.await {
                if user.is_admin {
                    return Ok(Admin(user));
                }
                return Err(ErrorForbidden(""));
            };
            Err(ErrorUnauthorized(""))
        })
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum APIError {
    FieldError { field_name: String, error: String },
    TextError { error: String },
}

#[derive(Serialize)]
pub struct APIAnswer {
    errors: Vec<APIError>,
}

impl APIAnswer {
    fn new() -> Self {
        Self { errors: Vec::new() }
    }

    fn add_field_error(&mut self, field_name: &str, error: &str) {
        self.errors.push(APIError::FieldError {
            field_name: field_name.to_owned(),
            error: error.to_owned(),
        });
    }

    fn add_text_error(&mut self, error: &str) {
        self.errors.push(APIError::TextError {
            error: error.to_owned(),
        });
    }

    fn is_ok(&self) -> bool {
        return self.errors.is_empty();
    }
}
