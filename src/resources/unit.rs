use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unit {
    pub(crate) id: i32,
    pub(crate) full_name: String,
    pub(crate) short_name: String
}
