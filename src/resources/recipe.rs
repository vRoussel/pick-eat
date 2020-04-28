use serde::{Deserialize, Serialize};
use super::ingredient::QuantifiedIngredient;
use super::category::DBCategory;
use super::tag::Tag;

#[derive(Debug, Serialize, Deserialize)]
pub struct Recipe {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) ingredients: Vec<QuantifiedIngredient>,
    pub(crate) categories: Vec<DBCategory>,
    pub(crate) tags: Vec<Tag>,
    pub(crate) prep_time_min: i32,
    pub(crate) cook_time_min: i32,
    pub(crate) image: Vec<u8>,
    pub(crate) publish_date: time::Date,
    pub(crate) instructions: Vec<String>
}

impl From<&tokio_postgres::row::Row> for Recipe {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        Recipe {
            id: row.get("id"),
            name: row.get("name"),
            desc: row.get("description"),
            ingredients: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions")
        }
    }
}
