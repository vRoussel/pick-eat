use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DBCategory {
    pub(crate) id: i32,
    pub(crate) name: String
}

#[derive(Debug, Deserialize)]
pub struct NewCategory {
    pub(crate) name: String
}

#[derive(Debug, Deserialize)]
pub struct CategoryUpdate {
    pub(crate) name: Option<String>
}

impl From<&tokio_postgres::row::Row> for DBCategory {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        DBCategory {
            id: row.get("id"),
            name: row.get("name")
        }
    }
}
