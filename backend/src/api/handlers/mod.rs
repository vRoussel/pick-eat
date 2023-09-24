use actix_web::{http::header, HttpResponse, HttpResponseBuilder};
use log::*;
use serde::Serialize;

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

pub fn set_and_log_json_body(
    mut builder: HttpResponseBuilder,
    value: impl Serialize,
) -> HttpResponse {
    match serde_json::to_string(&value) {
        Ok(v) => {
            debug!("{}", v);
            builder.insert_header((header::CONTENT_TYPE, "Application/json"));
            builder.body(v)
        }
        Err(e) => {
            error!("{}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
