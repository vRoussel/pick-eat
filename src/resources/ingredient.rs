use serde::{Deserialize, Serialize};
use super::unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) default_unit: Option<unit::FromDB>
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String,
    pub(crate) default_unit_id: Option<i32>
}

pub mod quantified {
    use serde::{Deserialize, Serialize};
    use super::unit;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Full {
        pub(crate) id: i32,
        pub(crate) name: String,
        pub(crate) quantity: Option<i16>,
        pub(crate) unit: Option<unit::FromDB>
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ref {
        pub(crate) id: i32,
        pub(crate) quantity: Option<i16>,
        pub(crate) unit_id: Option<i32>
    }
}


impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let default_unit = match row.try_get("default_unit_id") {
            Ok(unit_id) => Some(
                unit::FromDB {
                    id: unit_id,
                    full_name: row.get("default_unit_full_name"),
                    short_name: row.get("default_unit_short_name")
                }
            ),
            Err(_) => None
        };

        FromDB {
            id: row.get("id"),
            name: row.get("name"),
            default_unit: default_unit
        }
    }
}

impl From<&tokio_postgres::row::Row> for quantified::Full {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let unit = match row.try_get("unit_id") {
            Ok(unit_id) => Some(
                unit::FromDB {
                    id: unit_id,
                    full_name: row.get("unit_full_name"),
                    short_name: row.get("unit_short_name")
                }
            ),
            Err(_) => None
        };

        quantified::Full {
            id: row.get("id"),
            name: row.get("name"),
            quantity: row.get("quantity"),
            unit: unit
        }
    }
}
