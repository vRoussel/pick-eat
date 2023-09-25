use sqlx::Type;

use super::{InvalidInput, InvalidityKind};

#[derive(Debug, Type)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug)]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug)]
pub struct InvalidTag {
    pub name: Option<InvalidityKind>,
}

impl InvalidInput for InvalidTag {}
