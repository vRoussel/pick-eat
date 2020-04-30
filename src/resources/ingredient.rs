use serde::{Deserialize, Serialize};
use super::unit::Unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) default_unit: Unit
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuantifiedIngredient {
    id: i32,
    name: String,
    quantity: Option<i16>,
    unit: Option<Unit>
}

impl From<&tokio_postgres::row::Row> for Ingredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let default_unit = Unit {
            id: row.get("default_unit_id"),
            full_name: row.get("default_unit_full_name"),
            short_name: row.get("default_unit_short_name")
        };

        Ingredient {
            id: row.get("id"),
            name: row.get("name"),
            default_unit: default_unit
        }
    }
}

impl From<&tokio_postgres::row::Row> for QuantifiedIngredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let unit = match row.try_get("unit_id") {
            Ok(unit_id) => Some(
                Unit {
                    id: unit_id,
                    full_name: row.get("unit_full_name"),
                    short_name: row.get("unit_short_name")
                }
            ),
            Err(_) => None
        };

        QuantifiedIngredient {
            id: row.get("id"),
            name: row.get("name"),
            quantity: row.get("quantity"),
            unit: unit
        }
    }
}
