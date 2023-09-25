use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct CategoryOut {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct CategoryIn {
    name: String,
}

impl From<CategoryIn> for crate::models::NewCategory {
    fn from(c: CategoryIn) -> Self {
        Self { name: c.name }
    }
}

impl From<crate::models::Category> for CategoryOut {
    fn from(c: crate::models::Category) -> Self {
        Self {
            id: c.id,
            name: c.name,
        }
    }
}
