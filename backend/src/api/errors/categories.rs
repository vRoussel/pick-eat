use log::*;

use crate::models::{InvalidCategory, InvalidityKind};

use super::APIError;

impl From<InvalidCategory> for Vec<APIError> {
    fn from(value: InvalidCategory) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Une categorie avec ce nom existe déjà",
                        field: Some("name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Une categorie ne peut pas avoir un nom vide",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for category's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
