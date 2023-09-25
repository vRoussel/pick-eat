use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CredentialsIn {
    email: String,
    password: String,
}

impl From<CredentialsIn> for crate::models::Credentials {
    fn from(c: CredentialsIn) -> Self {
        Self {
            email: c.email,
            password: crate::models::Password::ClearText(c.password),
        }
    }
}
