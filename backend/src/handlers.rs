pub mod accounts;
pub mod categories;
pub mod ingredients;
pub mod recipes;
pub mod seasons;
pub mod sessions;
pub mod tags;
pub mod units;

use actix_web::HttpResponse;
use sqlx::error::DatabaseError;

pub fn db_error_to_http_response(db_error: &dyn DatabaseError) -> HttpResponse {
    match db_error.code().as_deref() {
        Some("23505") => HttpResponse::Conflict().finish(),
        Some("23503") => HttpResponse::UnprocessableEntity().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
