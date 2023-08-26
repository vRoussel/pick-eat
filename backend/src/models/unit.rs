use super::{InvalidInput, InvalidityKind};

#[derive(Debug)]
pub struct Unit {
    pub id: i32,
    pub full_name: String,
    pub short_name: String,
}

#[derive(Debug)]
pub struct NewUnit {
    pub full_name: String,
    pub short_name: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidUnit {
    pub full_name: Option<InvalidityKind>,
    pub short_name: Option<InvalidityKind>,
}

impl InvalidInput for InvalidUnit {}
