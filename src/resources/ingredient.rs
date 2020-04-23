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
    ingredient: Ingredient,
    quantity: i16,
    unit: Unit
}

impl From<&tokio_postgres::row::Row> for QuantifiedIngredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let unit = Unit {
            id: row.get("unit_id"),
            full_name: row.get("unit_full_name"),
            short_name: row.get("unit_short_name")
        };
        let default_unit = Unit {
            id: row.get("default_unit_id"),
            full_name: row.get("default_unit_full_name"),
            short_name: row.get("default_unit_short_name")
        };

        QuantifiedIngredient {
            ingredient: Ingredient {
                id: row.get("id"),
                name: row.get("name"),
                default_unit: default_unit
            },
            quantity: row.get("quantity"),
            unit: unit
        }
    }
}
