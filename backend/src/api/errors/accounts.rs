use log::*;

use crate::models::{self, InvalidAccountUpdate, InvalidNewAccount};

use super::APIError;

fn display_name_invalidity_to_api_error(
    display_name_inv: models::InvalidityKind,
) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match display_name_inv {
        Kind::AlreadyUsed => Some(APIError {
            message: "Ce pseudo est déjà pris",
            field: Some("display_name"),
            code: None,
        }),
        Kind::Empty => Some(APIError {
            message: "Un pseudo ne peut pas être vide",
            field: Some("display_name"),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's display_name, this should not happen",
                display_name_inv
            );
            None
        }
    }
}

fn email_invalidity_to_api_error(email_inv: models::InvalidityKind) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match email_inv {
        Kind::AlreadyUsed => Some(APIError {
            message: "Cet email est déjà utilisé",
            field: Some("email"),
            code: None,
        }),
        Kind::Empty => Some(APIError {
            message: "Veuillez saisir une adresse mail",
            field: Some("email"),
            code: None,
        }),
        Kind::BadFormat => Some(APIError {
            message: "Cet email est invalide",
            field: Some("email"),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's email, this should not happen",
                email_inv
            );
            None
        }
    }
}

fn password_invalidity_to_api_error(
    password_inv: models::InvalidityKind,
    field_name: &'static str,
) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match password_inv {
        Kind::TooShort => Some(APIError {
            message: "Le mot de passe doit contenir au moins 8 caractères",
            field: Some(field_name),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's {}, this should not happen",
                password_inv, field_name
            );
            None
        }
    }
}

impl From<InvalidNewAccount> for Vec<APIError> {
    fn from(value: InvalidNewAccount) -> Self {
        let mut ret = Vec::new();
        if let Some(v) = value.display_name {
            display_name_invalidity_to_api_error(v).map(|e| ret.push(e));
        };
        if let Some(v) = value.email {
            email_invalidity_to_api_error(v).map(|e| ret.push(e));
        }
        if let Some(v) = value.password {
            password_invalidity_to_api_error(v, "password").map(|e| ret.push(e));
        }
        ret
    }
}

impl From<InvalidAccountUpdate> for Vec<APIError> {
    fn from(value: InvalidAccountUpdate) -> Self {
        let mut ret = Vec::new();
        if let Some(v) = value.display_name {
            display_name_invalidity_to_api_error(v).map(|e| ret.push(e));
        };
        if let Some(v) = value.email {
            email_invalidity_to_api_error(v).map(|e| ret.push(e));
        }
        if let Some(v) = value.new_password {
            password_invalidity_to_api_error(v, "new_password").map(|e| ret.push(e));
        }
        if let Some(v) = value.old_password {
            match v {
                models::InvalidityKind::Empty => {
                    ret.push(APIError {
                        message: "Le mot de passe actuel doit être renseigné",
                        field: Some("old_password"),
                        code: None,
                    });
                }
                models::InvalidityKind::Mismatch => {
                    ret.push(APIError {
                        message: "Le mot de passe est incorrect",
                        field: Some("old_password"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for accountupdate's old password, this should not happen",
                        v
                    );
                }
            };
        }
        ret
    }
}
