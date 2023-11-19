use serde::Serialize;

use super::{
    CategoryOut, DietOut, IngredientOut, PublicAccountDataOut, SeasonOut, TagOut, UnitOut,
};

#[derive(Debug, Serialize)]
pub struct BundleOut {
    pub tags: Vec<TagOut>,
    pub categories: Vec<CategoryOut>,
    pub ingredients: Vec<IngredientOut>,
    pub units: Vec<UnitOut>,
    pub seasons: Vec<SeasonOut>,
    pub diets: Vec<DietOut>,
    pub accounts_with_recipes: Vec<PublicAccountDataOut>,
}
