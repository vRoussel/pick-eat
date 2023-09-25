use log::*;

use crate::models::{InvalidTag, InvalidityKind};

use super::APIError;

impl From<InvalidTag> for Vec<APIError> {
    fn from(value: InvalidTag) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Un tag avec ce nom existe déjà",
                        field: Some("name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un tag ne peut pas avoir un nom vide",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for tag's name, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
