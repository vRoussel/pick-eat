use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) full_name: String,
    pub(crate) short_name: String
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) full_name: String,
    pub(crate) short_name: String
}

impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            full_name: row.get("full_name"),
            short_name: row.get("short_name")
        }
    }
}
