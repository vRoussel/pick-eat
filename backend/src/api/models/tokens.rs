use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TokenNewRequestIn {
    email: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountValidationTokenUseRequestIn {
    token: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PasswordResetTokenUseRequestIn {
    token: String,
    new_password: String,
}

impl From<TokenNewRequestIn> for crate::models::TokenNewRequest {
    fn from(t: TokenNewRequestIn) -> Self {
        Self { email: t.email }
    }
}

impl From<AccountValidationTokenUseRequestIn> for crate::models::AccountValidationTokenUseRequest {
    fn from(t: AccountValidationTokenUseRequestIn) -> Self {
        Self { token: t.token }
    }
}

impl From<PasswordResetTokenUseRequestIn> for crate::models::PasswordResetTokenUseRequest {
    fn from(t: PasswordResetTokenUseRequestIn) -> Self {
        Self {
            token: t.token,
            new_password: crate::models::Password::ClearText(t.new_password),
        }
    }
}
