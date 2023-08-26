use super::{InvalidInput, InvalidityKind};

#[derive(Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct NewCategory {
    pub name: String,
}

#[derive(Debug)]
pub struct InvalidCategory {
    pub name: Option<InvalidityKind>,
}

impl InvalidInput for InvalidCategory {}
