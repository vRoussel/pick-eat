use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub(crate) id: i32,
    pub(crate) name: String
}

impl From<&tokio_postgres::row::Row> for Category {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        Category {
            id: row.get("id"),
            name: row.get("name")
        }
    }
}
