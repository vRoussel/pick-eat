use serde::{Deserialize, Serialize};
use super::unit::DBUnit;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBIngredient {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) default_unit: Option<DBUnit>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DBQuantifiedIngredient {
    id: i32,
    name: String,
    quantity: Option<i16>,
    unit: Option<DBUnit>
}

impl From<&tokio_postgres::row::Row> for DBIngredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let default_unit = match row.try_get("default_unit_id") {
            Ok(unit_id) => Some(
                DBUnit {
                    id: unit_id,
                    full_name: row.get("default_unit_full_name"),
                    short_name: row.get("default_unit_short_name")
                }
            ),
            Err(_) => None
        };

        DBIngredient {
            id: row.get("id"),
            name: row.get("name"),
            default_unit: default_unit
        }
    }
}

impl From<&tokio_postgres::row::Row> for DBQuantifiedIngredient {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let unit = match row.try_get("unit_id") {
            Ok(unit_id) => Some(
                DBUnit {
                    id: unit_id,
                    full_name: row.get("unit_full_name"),
                    short_name: row.get("unit_short_name")
                }
            ),
            Err(_) => None
        };

        DBQuantifiedIngredient {
            id: row.get("id"),
            name: row.get("name"),
            quantity: row.get("quantity"),
            unit: unit
        }
    }
}
