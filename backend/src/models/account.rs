use super::{InvalidInput, InvalidityKind, Password};

#[derive(Debug)]
pub struct Account {
    pub id: i32,
    pub password: String,
    pub display_name: String,
    pub email: String,
    pub creation_date: chrono::NaiveDate,
    pub is_admin: bool,
    pub is_validated: bool,
}

#[derive(Debug)]
pub struct PublicAccountData {
    pub id: i32,
    pub display_name: String,
}

#[derive(Debug)]
pub struct NewAccount {
    pub display_name: String,
    pub email: String,
    pub password: Password,
}

#[derive(Debug)]
pub struct AccountUpdate {
    pub display_name: String,
    pub email: String,
    pub old_password: Password,
    pub new_password: Option<Password>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidNewAccount {
    pub display_name: Option<InvalidityKind>,
    pub email: Option<InvalidityKind>,
    pub password: Option<InvalidityKind>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidAccountUpdate {
    pub display_name: Option<InvalidityKind>,
    pub email: Option<InvalidityKind>,
    pub old_password: Option<InvalidityKind>,
    pub new_password: Option<InvalidityKind>,
}

impl InvalidInput for InvalidNewAccount {}
impl InvalidInput for InvalidAccountUpdate {}
