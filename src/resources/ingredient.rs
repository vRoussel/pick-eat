use serde::{Deserialize, Serialize};
use super::unit::Unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBIngredient {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) default_unit: Unit
}

impl From<&tokio_postgres::row::Row> for DBIngredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let default_unit = Unit {
            id: row.get("default_unit_id"),
            full_name: row.get("default_unit_full_name"),
            short_name: row.get("default_unit_short_name")
        };

        DBIngredient {
            id: row.get("id"),
            name: row.get("name"),
            default_unit: default_unit
        }
    }
}

