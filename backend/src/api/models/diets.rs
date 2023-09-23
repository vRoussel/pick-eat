use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct DietOut {
    id: i32,
    name: String,
    label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DietIn {
    name: String,
    label: Option<String>,
}

impl From<DietIn> for crate::models::NewDiet {
    fn from(d: DietIn) -> Self {
        Self {
            name: d.name,
            label: d.label,
        }
    }
}

impl From<crate::models::Diet> for DietOut {
    fn from(d: crate::models::Diet) -> Self {
        Self {
            id: d.id,
            name: d.name,
            label: d.label,
        }
    }
}
