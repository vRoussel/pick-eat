use serde::{Deserialize, Serialize};

use super::UnitOut;

#[derive(Debug, Serialize)]
pub struct IngredientOut {
    id: i32,
    name: String,
    default_unit: Option<UnitOut>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IngredientIn {
    name: String,
    default_unit_id: Option<i32>,
}

impl From<IngredientIn> for crate::models::NewIngredient {
    fn from(i: IngredientIn) -> Self {
        Self {
            name: i.name,
            default_unit_id: i.default_unit_id,
        }
    }
}

impl From<crate::models::Ingredient> for IngredientOut {
    fn from(i: crate::models::Ingredient) -> Self {
        Self {
            id: i.id,
            name: i.name,
            default_unit: i.default_unit.map(|u| u.into()),
        }
    }
}
