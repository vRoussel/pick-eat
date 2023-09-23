use log::*;

use crate::models::{InvalidIngredient, InvalidityKind};

use super::APIError;

impl From<InvalidIngredient> for Vec<APIError> {
    fn from(value: InvalidIngredient) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Un ingredient avec ce nom existe déjà",
                        field: Some("name"),
                        code: None,
                    });
                }
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Un ingredient ne peut pas avoir un nom vide",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for ingredient's name, this should not happen",
                        v
                    );
                }
            };
        }
        if let Some(v) = value.default_unit_id {
            match v {
                Kind::InvalidRef => {
                    ret.push(APIError {
                        message: "Cette unité est invalide, veuillez en choisir une autre",
                        field: Some("default_unit_id"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for ingredient's default_unit_id, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
