use log::*;

use crate::models::{InvalidRecipe, InvalidityKind};

use super::APIError;

impl From<InvalidRecipe> for Vec<APIError> {
    fn from(value: InvalidRecipe) -> Self {
        type Kind = InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez donner un nom à votre recette",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.preparation_time_min {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le temps de préparation doit être d'au moins 1 minute",
                        field: Some("prep_time_min"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's prep_time_min, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.cooking_time_min {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le temps de cuisson ne peut pas être négatif",
                        field: Some("cook_time_min"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's cook_time_min, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.n_shares {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le nombre de part doit être au moins de 1",
                        field: Some("n_shares"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's n_shares, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.shares_unit {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "La dénomination des parts est obligatoire",
                        field: Some("shares_unit"),
                        code: None,
                    });
                }
                Kind::TooLong => {
                    ret.push(APIError {
                        message: "La dénomination des parts ne doit pas dépasser 15 caractères",
                        field: Some("shares_unit"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's shares_unit, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.categories {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez selectionner au moins une catégorie",
                        field: Some("category_ids"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's categories, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.seasons {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez selectionner au moins une saison",
                        field: Some("season_ids"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's seasons, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.ingredients {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez ajouter au moins un ingrédient",
                        field: Some("q_ingredients"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's ingredients, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.instructions {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez renseigner les instructions pour réaliser la recette",
                        field: Some("instructions"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's categories, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}
