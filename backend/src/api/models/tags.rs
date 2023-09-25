use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TagOut {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagIn {
    name: String,
}

impl From<TagIn> for crate::models::NewTag {
    fn from(t: TagIn) -> Self {
        Self { name: t.name }
    }
}

impl From<crate::models::Tag> for TagOut {
    fn from(t: crate::models::Tag) -> Self {
        Self {
            id: t.id,
            name: t.name,
        }
    }
}
