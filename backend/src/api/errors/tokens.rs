use log::*;

use crate::models::{
    InvalidAccountValidationTokenUseRequest, InvalidPasswordResetTokenUseRequest,
    InvalidTokenNewRequest, InvalidityKind,
};

use super::APIError;

impl From<InvalidTokenNewRequest> for Vec<APIError> {
    fn from(value: InvalidTokenNewRequest) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.email {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez saisir une adresse mail",
                        field: Some("email"),
                        code: None,
                    });
                }
                Kind::BadFormat => {
                    ret.push(APIError {
                        message: "Cette adresse mail est invalide",
                        field: Some("email"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}

impl From<InvalidAccountValidationTokenUseRequest> for Vec<APIError> {
    fn from(value: InvalidAccountValidationTokenUseRequest) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.token {
            match v {
                Kind::Expired | Kind::NotFound => {
                    ret.push(APIError {
                        message: "Votre lien de validation est invalide ou expiré, veuillez en générer un nouveau en essayant de vous connecter",
                        field: None,
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}

impl From<InvalidPasswordResetTokenUseRequest> for Vec<APIError> {
    fn from(value: InvalidPasswordResetTokenUseRequest) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.token {
            match v {
                Kind::Expired | Kind::NotFound => {
                    ret.push(APIError {
                        message: "Votre lien de validation est invalide ou expiré, veuillez en générer un nouveau en essayant de vous connecter",
                        field: None,
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tokenRequest's name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.new_password {
            match v {
                Kind::TooShort => Some(APIError {
                    message: "Le mot de passe doit contenir au moins 8 caractères",
                    field: Some("new_password"),
                    code: None,
                }),
                _ => {
                    warn!(
                        "{:?} error received for password_reset_token_use's new_password, this should not happen",
                        v
                    );
                    None
                }
            };
        };
        ret
    }
}
