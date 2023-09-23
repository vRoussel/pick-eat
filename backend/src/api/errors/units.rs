use log::*;

use crate::models::{InvalidUnit, InvalidityKind};

use super::APIError;

impl From<InvalidUnit> for Vec<APIError> {
    fn from(value: InvalidUnit) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.full_name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Une unité avec ce nom avec ce nom existe déjà",
                        field: Some("full_name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un unité ne peut pas avoir un nom vide",
                        field: Some("full_name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for unit's full name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.short_name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Une unité avec cette abréviation existe déjà",
                        field: Some("short_name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un unité ne peut pas avoir une abbréviation vide",
                        field: Some("short_name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for unit's short name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
