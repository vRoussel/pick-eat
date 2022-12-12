use super::unit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    id: i32,
    name: String,
    quantity: Option<f32>,
    unit: Option<unit::FromDB>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Ref {
    pub id: i32,
    pub quantity: Option<f32>,
    pub unit_id: Option<unit::Ref>,
}
