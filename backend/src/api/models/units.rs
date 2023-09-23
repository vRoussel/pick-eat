use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct UnitOut {
    id: i32,
    full_name: String,
    short_name: String,
}

#[derive(Debug, Deserialize)]
pub struct UnitIn {
    full_name: String,
    short_name: Option<String>,
}

impl From<UnitIn> for crate::models::NewUnit {
    fn from(u: UnitIn) -> Self {
        Self {
            full_name: u.full_name,
            short_name: u.short_name,
        }
    }
}

impl From<crate::models::Unit> for UnitOut {
    fn from(u: crate::models::Unit) -> Self {
        Self {
            id: u.id,
            full_name: u.full_name,
            short_name: u.short_name,
        }
    }
}
