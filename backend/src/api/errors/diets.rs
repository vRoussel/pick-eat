use log::*;

use crate::models::{InvalidDiet, InvalidityKind};

use super::APIError;

impl From<InvalidDiet> for Vec<APIError> {
    fn from(value: InvalidDiet) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Un régime alimentaire avec ce nom existe déjà",
                        field: Some("name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un régime alimentaire ne peut pas avoir un nom vide",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for diet's name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.label {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Un régime alimentaire avec ce label existe déjà",
                        field: Some("label"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un régime alimentaire ne peut pas avoir un label vide",
                        field: Some("label"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for diet's label, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
