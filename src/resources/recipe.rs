use serde::{Deserialize, Serialize};
use super::category;
use super::tag;
use super::ingredient;
use super::ingredient::quantified as QIngredient;

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) q_ingredients: Vec<QIngredient::Full>,
    pub(crate) categories: Vec<category::FromDB>,
    pub(crate) tags: Vec<tag::FromDB>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: Vec<u8>,
    pub(crate) publish_date: time::Date,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String,
    pub(crate) desc: String,
    pub(crate) q_ingredient_ids: Vec<QIngredient::Ref>,
    pub(crate) category_ids: Vec<i32>,
    pub(crate) tag_ids: Vec<i32>,
    pub(crate) prep_time_min: i16,
    pub(crate) cook_time_min: i16,
    pub(crate) image: Vec<u8>,
    pub(crate) instructions: Vec<String>,
    pub(crate) n_shares: i16
}


impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        FromDB {
            id: row.get("id"),
            name: row.get("name"),
            desc: row.get("description"),
            q_ingredients: Vec::new(),
            categories: Vec::new(),
            tags: Vec::new(),
            prep_time_min: row.get("preparation_time_min"),
            cook_time_min: row.get("cooking_time_min"),
            image: row.get("image"),
            publish_date: row.get("publication_date"),
            instructions: row.get("instructions"),
            n_shares: row.get("n_shares")
        }
    }
}
