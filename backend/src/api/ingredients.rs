use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::{Deserialize, Serialize};

use super::units::UnitOut;
use super::{APIError, Admin, User};
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidIngredient};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_ingredients)
        .service(add_ingredient)
        .service(get_ingredient)
        .service(replace_ingredient)
        .service(delete_ingredient);
}

impl From<InvalidIngredient> for Vec<APIError> {
    fn from(value: InvalidIngredient) -> Self {
        type Kind = models::InvalidityKind;
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

#[derive(Debug, Serialize)]
struct IngredientOut {
    id: i32,
    name: String,
    default_unit: Option<UnitOut>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IngredientIn {
    name: String,
    default_unit_id: Option<i32>,
}

impl From<IngredientIn> for models::NewIngredient {
    fn from(i: IngredientIn) -> Self {
        Self {
            name: i.name,
            default_unit_id: i.default_unit_id,
        }
    }
}

impl From<models::Ingredient> for IngredientOut {
    fn from(i: models::Ingredient) -> Self {
        Self {
            id: i.id,
            name: i.name,
            default_unit: i.default_unit.map(|u| u.into()),
        }
    }
}

#[get("/ingredients")]
async fn get_all_ingredients(app: web::Data<App>) -> impl Responder {
    let ingredients: Vec<IngredientOut> = match app.get_all_ingredients().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&ingredients) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(ingredients)
}

#[post("/ingredients")]
async fn add_ingredient(
    new_ingredient: web::Json<IngredientIn>,
    app: web::Data<App>,
    _user: User,
) -> impl Responder {
    debug!("{:?}", new_ingredient);

    let mut i: models::NewIngredient = new_ingredient.into_inner().into();

    let new_id = match app.add_ingredient(&mut i).await {
        Ok(v) => v,
        Err(e) => {
            use AppErrorWith::*;
            if let AppError(inner_e) = &e {
                error!("{}", inner_e);
            }
            return e.into();
        }
    };

    debug!("{}", new_id);
    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[get("/ingredients/{id}")]
async fn get_ingredient(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let ingredient: IngredientOut = match app.get_ingredient(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&ingredient) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(ingredient)
}

#[put("/ingredients/{id}")]
async fn replace_ingredient(
    id: web::Path<i32>,
    new_ingredient: web::Json<IngredientIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_ingredient);

    let mut i: models::NewIngredient = new_ingredient.into_inner().into();
    match app.replace_ingredient(id.into_inner(), &mut i).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            use AppErrorWith::*;
            if let AppError(inner_e) = &e {
                error!("{}", inner_e);
            }
            return e.into();
        }
    }

    HttpResponse::Ok().finish()
}

#[delete("/ingredients/{id}")]
async fn delete_ingredient(
    id: web::Path<i32>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    match app.delete_ingredient(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
