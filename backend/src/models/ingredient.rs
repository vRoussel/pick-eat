use super::{InvalidInput, InvalidityKind};

#[derive(Debug)]
pub struct Ingredient {
    pub id: i32,
    pub name: String,
    pub default_unit: Option<super::Unit>,
}

#[derive(Debug)]
pub struct NewIngredient {
    pub name: String,
    pub default_unit_id: Option<i32>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidIngredient {
    pub name: Option<InvalidityKind>,
    pub default_unit_id: Option<InvalidityKind>,
}

impl InvalidInput for InvalidIngredient {}
