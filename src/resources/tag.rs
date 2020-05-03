use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBTag {
    pub(crate) id: i32,
    pub(crate) name: String
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewTag {
    pub(crate) name: String
}

impl From<&tokio_postgres::row::Row> for DBTag {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        DBTag {
            id: row.get("id"),
            name: row.get("name")
        }
    }
}
