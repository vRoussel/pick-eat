use serde::{Deserialize, Serialize};

use super::{CategoryOut, DietOut, SeasonOut, TagOut, UnitOut};

#[derive(Debug, Serialize)]
pub struct RecipeOut {
    id: i32,
    name: String,
    notes: String,
    prep_time_min: i16,
    cook_time_min: i16,
    image: String,
    publication_date: chrono::NaiveDate,
    instructions: Vec<String>,
    n_shares: i16,
    shares_unit: String,
    is_favorite: bool,
    is_private: bool,
    q_ingredients: Vec<QIngredientOut>,
    categories: Vec<CategoryOut>,
    tags: Vec<TagOut>,
    seasons: Vec<SeasonOut>,
    author_id: i32,
    author_name: String,
    diets: Vec<DietOut>,
}

#[derive(Debug, Serialize)]
pub struct RecipeSummaryOut {
    id: i32,
    name: String,
    image: String,
    n_shares: i16,
    shares_unit: String,
    is_favorite: bool,
    is_private: bool,
    q_ingredients: Vec<QIngredientOut>,
    diets: Vec<DietOut>,
}

#[derive(Debug, Serialize)]
struct QIngredientOut {
    id: i32,
    name: String,
    quantity: Option<f32>,
    unit: Option<UnitOut>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeIn {
    name: String,
    notes: String,
    prep_time_min: i16,
    cook_time_min: i16,
    image: String,
    instructions: Vec<String>,
    n_shares: i16,
    shares_unit: String,
    is_private: bool,
    q_ingredients: Vec<QIngredientIn>,
    category_ids: Vec<i32>,
    tag_ids: Vec<i32>,
    season_ids: Vec<i32>,
    diet_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct QIngredientIn {
    id: i32,
    quantity: Option<f32>,
    unit_id: Option<i32>,
}

impl From<crate::models::Recipe> for RecipeOut {
    fn from(r: crate::models::Recipe) -> Self {
        Self {
            id: r.id,
            name: r.name,
            notes: r.notes,
            prep_time_min: r.preparation_time_min,
            cook_time_min: r.cooking_time_min,
            image: r.image,
            publication_date: r.publication_date,
            instructions: r.instructions,
            n_shares: r.n_shares,
            shares_unit: r.shares_unit,
            is_favorite: r.is_favorite,
            is_private: r.is_private,
            q_ingredients: r.ingredients.into_iter().map(|i| i.into()).collect(),
            categories: r.categories.into_iter().map(|c| c.into()).collect(),
            tags: r.tags.into_iter().map(|t| t.into()).collect(),
            seasons: r.seasons.into_iter().map(|s| s.into()).collect(),
            author_id: r.author_id,
            author_name: r.author_name,
            diets: r.diets.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl From<crate::models::RecipeSummary> for RecipeSummaryOut {
    fn from(r: crate::models::RecipeSummary) -> Self {
        Self {
            id: r.id,
            name: r.name,
            image: r.image,
            n_shares: r.n_shares,
            shares_unit: r.shares_unit,
            is_favorite: r.is_favorite,
            is_private: r.is_private,
            q_ingredients: r.ingredients.into_iter().map(|i| i.into()).collect(),
            diets: r.diets.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl From<crate::models::QIngredient> for QIngredientOut {
    fn from(i: crate::models::QIngredient) -> Self {
        Self {
            id: i.id,
            name: i.name,
            quantity: i.quantity,
            unit: i.unit.map(|u| u.into()),
        }
    }
}

impl From<QIngredientIn> for crate::models::NewQIngredient {
    fn from(value: QIngredientIn) -> Self {
        Self {
            id: value.id,
            quantity: value.quantity,
            unit_id: value.unit_id,
        }
    }
}

impl From<RecipeIn> for crate::models::NewRecipe {
    fn from(value: RecipeIn) -> Self {
        Self {
            name: value.name,
            notes: value.notes,
            preparation_time_min: value.prep_time_min,
            cooking_time_min: value.cook_time_min,
            image: value.image,
            instructions: value.instructions,
            n_shares: value.n_shares,
            shares_unit: value.shares_unit,
            is_private: value.is_private,
            ingredients: value.q_ingredients.into_iter().map(|i| i.into()).collect(),
            categories: value.category_ids,
            tags: value.tag_ids,
            seasons: value.season_ids,
            diets: value.diet_ids,
        }
    }
}
