use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::{Deserialize, Serialize};

use super::{APIError, Admin};
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidDiet};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_diets)
        .service(add_diet)
        .service(get_diet)
        .service(replace_diet)
        .service(delete_diet);
}

impl From<InvalidDiet> for Vec<APIError> {
    fn from(value: InvalidDiet) -> Self {
        type Kind = models::InvalidityKind;
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

#[derive(Debug, Serialize)]
pub struct DietOut {
    id: i32,
    name: String,
    label: Option<String>,
}

#[derive(Debug, Deserialize)]
struct DietIn {
    name: String,
    label: Option<String>,
}

impl From<DietIn> for models::NewDiet {
    fn from(d: DietIn) -> Self {
        Self {
            name: d.name,
            label: d.label,
        }
    }
}

impl From<models::Diet> for DietOut {
    fn from(d: models::Diet) -> Self {
        Self {
            id: d.id,
            name: d.name,
            label: d.label,
        }
    }
}

#[get("/diets")]
async fn get_all_diets(app: web::Data<App>) -> impl Responder {
    let diets: Vec<DietOut> = match app.get_all_diets().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&diets) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(diets)
}

#[post("/diets")]
async fn add_diet(
    new_diet: web::Json<DietIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_diet);

    let mut t: models::NewDiet = new_diet.into_inner().into();

    let new_id = match app.add_diet(&mut t).await {
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

#[get("/diets/{id}")]
async fn get_diet(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let diet: DietOut = match app.get_diet(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&diet) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(diet)
}

#[put("/diets/{id}")]
async fn replace_diet(
    id: web::Path<i32>,
    new_diet: web::Json<DietIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_diet);

    let mut t: models::NewDiet = new_diet.into_inner().into();
    match app.replace_diet(id.into_inner(), &mut t).await {
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

#[delete("/diets/{id}")]
async fn delete_diet(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_diet(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
