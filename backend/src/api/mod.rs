pub mod ingredients;
pub mod categories;
pub mod diets;
pub mod seasons;
pub mod tags;
pub mod units;

use std::convert::{TryFrom, TryInto};
use std::{future::Future, pin::Pin};

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorUnauthorized},
    FromRequest, HttpRequest,
};
use actix_web::{Error, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::app::{AppError, AppErrorWith};
use crate::models;

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

#[derive(Serialize, Default)]
pub struct APIError {
    message: &'static str,
    field: Option<&'static str>,
    code: Option<&'static str>,
}

#[derive(Serialize)]
pub struct APIAnswer {
    errors: Vec<APIError>,
}

impl<T> From<AppErrorWith<T>> for HttpResponse
where
    T: models::InvalidInput,
{
    fn from(value: AppErrorWith<T>) -> Self {
        match value {
            AppErrorWith::InvalidInput(v) => {
                HttpResponse::BadRequest().json(APIAnswer { errors: v.into() })
            }
            AppErrorWith::AppError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl From<AppError> for HttpResponse {
    fn from(value: AppError) -> Self {
        match value {
            AppError::InternalError(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl<T> TryFrom<AppErrorWith<T>> for APIAnswer
where
    T: models::InvalidInput,
{
    type Error = ();
    fn try_from(value: AppErrorWith<T>) -> Result<APIAnswer, ()> {
        match value {
            AppErrorWith::InvalidInput(v) => Ok(APIAnswer { errors: v.into() }),
            AppErrorWith::AppError(v) => v.try_into(),
        }
    }
}

impl TryFrom<AppError> for APIAnswer {
    type Error = ();
    fn try_from(value: AppError) -> Result<APIAnswer, ()> {
        match value {
            AppError::InternalError(_) => Err(()),
        }
    }
}
