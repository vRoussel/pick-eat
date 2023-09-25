mod accounts;
mod categories;
mod diets;
mod ingredients;
mod recipes;
mod tags;
mod tokens;
mod units;

use serde::Serialize;

#[derive(Serialize, Default)]
pub struct APIError {
    pub message: &'static str,
    pub field: Option<&'static str>,
    pub code: Option<&'static str>,
}

#[derive(Serialize)]
pub struct APIAnswer {
    pub errors: Vec<APIError>,
}

impl From<APIError> for APIAnswer {
    fn from(value: APIError) -> Self {
        Self {
            errors: vec![value],
        }
    }
}
