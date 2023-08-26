use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::{Deserialize, Serialize};

use super::{APIError, Admin};
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidUnit};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_units)
        .service(add_unit)
        .service(get_unit)
        .service(replace_unit)
        .service(delete_unit);
}

impl From<InvalidUnit> for Vec<APIError> {
    fn from(value: InvalidUnit) -> Self {
        type Kind = models::InvalidityKind;
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
            };
        };
        ret
    }
}

#[derive(Debug, Serialize)]
struct UnitOut {
    id: i32,
    full_name: String,
    short_name: String,
}

#[derive(Debug, Deserialize)]
struct UnitIn {
    full_name: String,
    short_name: Option<String>,
}

impl From<UnitIn> for models::NewUnit {
    fn from(u: UnitIn) -> Self {
        Self {
            full_name: u.full_name,
            short_name: u.short_name,
        }
    }
}

impl From<models::Unit> for UnitOut {
    fn from(u: models::Unit) -> Self {
        Self {
            id: u.id,
            full_name: u.full_name,
            short_name: u.short_name,
        }
    }
}

#[get("/units")]
async fn get_all_units(app: web::Data<App>) -> impl Responder {
    let units: Vec<UnitOut> = match app.get_all_units().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&units) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(units)
}

#[post("/units")]
async fn add_unit(
    new_unit: web::Json<UnitIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_unit);

    let mut t: models::NewUnit = new_unit.into_inner().into();

    let new_id = match app.add_unit(&mut t).await {
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

#[get("/units/{id}")]
async fn get_unit(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let unit: UnitOut = match app.get_unit(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&unit) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(unit)
}

#[put("/units/{id}")]
async fn replace_unit(
    id: web::Path<i32>,
    new_unit: web::Json<UnitIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_unit);

    let mut t: models::NewUnit = new_unit.into_inner().into();
    match app.replace_unit(id.into_inner(), &mut t).await {
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

#[delete("/units/{id}")]
async fn delete_unit(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_unit(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
