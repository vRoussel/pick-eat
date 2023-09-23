pub mod errors;
pub mod handlers;
pub mod models;
pub mod query_params;

use core::fmt;
use std::{future::Future, pin::Pin};

use actix_identity::Identity;
use actix_session::Session;
use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorUnauthorized},
    FromRequest, HttpRequest,
};
use actix_web::{Error, HttpResponse};
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{
    app::{AppError, AppErrorWith},
    models::Range,
};

use self::{errors::APIAnswer, query_params::QueryParamError};

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

impl<T> From<AppErrorWith<T>> for HttpResponse
where
    T: crate::models::InvalidInput,
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
            AppError::StorageError(_) => HttpResponse::InternalServerError().finish(),
            AppError::PasswordHashingError(_) => HttpResponse::InternalServerError().finish(),
            AppError::Other(_) => HttpResponse::InternalServerError().finish(),
            AppError::InvalidInputParam(_) => HttpResponse::BadRequest().finish(),
            AppError::NotAllowed => HttpResponse::Forbidden().finish(),
        }
    }
}

impl From<QueryParamError> for HttpResponse {
    fn from(value: QueryParamError) -> Self {
        HttpResponse::BadRequest().body(value.to_string())
    }
}

struct RangeVisitor;
impl<'de> Visitor<'de> for RangeVisitor {
    type Value = Range;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a strictly positive range such as 1-10")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let pair: Vec<i64> = value
            .split('-')
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()
            .map_err(|e| E::custom(e))?;

        if pair.len() != 2 {
            return Err(E::custom("not a valid range format"));
        }

        Range::new(pair[0], pair[1]).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Range, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RangeVisitor)
    }
}
