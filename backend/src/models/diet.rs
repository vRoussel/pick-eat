use super::{InvalidInput, InvalidityKind};

#[derive(Debug)]
pub struct Diet {
    pub id: i32,
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug)]
pub struct NewDiet {
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidDiet {
    pub name: Option<InvalidityKind>,
    pub label: Option<InvalidityKind>,
}

impl InvalidInput for InvalidDiet {}
