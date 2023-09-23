use serde::{Deserialize, Serialize};

use crate::models;

#[derive(Debug, Serialize)]
pub struct AccountOut {
    id: i32,
    display_name: String,
    email: String,
    creation_date: chrono::NaiveDate,
    is_admin: bool,
    is_validated: bool,
}

#[derive(Debug, Serialize)]
pub struct PublicAccountDataOut {
    id: i32,
    display_name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewAccountIn {
    display_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AccountUpdateIn {
    display_name: String,
    email: String,
    old_password: String,
    new_password: Option<String>,
}

impl std::fmt::Debug for NewAccountIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NewAccountIn {{ display_name: {}, email: <hidden>, password: <hidden> }}",
            self.display_name
        )
    }
}

impl std::fmt::Debug for AccountUpdateIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountUpdateIn {{ display_name: {}, email: <hidden>, old_password: <hidden>, new_password: <hidden> }}",
            self.display_name
        )
    }
}

impl From<NewAccountIn> for models::NewAccount {
    fn from(a: NewAccountIn) -> Self {
        Self {
            display_name: a.display_name,
            email: a.email,
            password: models::Password::ClearText(a.password),
        }
    }
}

impl From<AccountUpdateIn> for models::AccountUpdate {
    fn from(a: AccountUpdateIn) -> Self {
        Self {
            display_name: a.display_name,
            email: a.email,
            old_password: models::Password::ClearText(a.old_password),
            new_password: a.new_password.map(|v| models::Password::ClearText(v)),
        }
    }
}

impl From<models::Account> for AccountOut {
    fn from(a: models::Account) -> Self {
        Self {
            id: a.id,
            display_name: a.display_name,
            email: a.email,
            creation_date: a.creation_date,
            is_admin: a.is_admin,
            is_validated: a.is_validated,
        }
    }
}

impl From<models::PublicAccountData> for PublicAccountDataOut {
    fn from(a: models::PublicAccountData) -> Self {
        Self {
            id: a.id,
            display_name: a.display_name,
        }
    }
}
