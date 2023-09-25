use chrono::prelude::*;

use super::{InvalidInput, InvalidityKind, Password};

#[derive(Debug)]
pub struct Token {
    pub token: String,
    pub account_id: i32,
    pub valid_until: DateTime<Utc>,
}

#[derive(Debug)]
pub struct TokenNewRequest {
    pub email: String,
}

#[derive(Debug)]
pub struct AccountValidationTokenUseRequest {
    pub token: String,
}

#[derive(Debug)]
pub struct PasswordResetTokenUseRequest {
    pub token: String,
    pub new_password: Password,
}

pub enum TokenType {
    AccountValidation,
    PasswordReset,
}

#[derive(Debug)]
pub struct InvalidTokenNewRequest {
    pub email: Option<InvalidityKind>,
}

#[derive(Debug)]
pub struct InvalidAccountValidationTokenUseRequest {
    pub token: Option<InvalidityKind>,
}

#[derive(Debug, Default)]
pub struct InvalidPasswordResetTokenUseRequest {
    pub token: Option<InvalidityKind>,
    pub new_password: Option<InvalidityKind>,
}

impl InvalidInput for InvalidTokenNewRequest {}
impl InvalidInput for InvalidAccountValidationTokenUseRequest {}
impl InvalidInput for InvalidPasswordResetTokenUseRequest {}
