use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub(crate) id: i32,
    pub(crate) name: String
}

impl From<&tokio_postgres::row::Row> for Tag {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        Tag {
            id: row.get("id"),
            name: row.get("name")
        }
    }
}
