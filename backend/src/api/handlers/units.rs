use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::api::handlers::set_and_log_json_body;
use crate::api::{models, Admin, User};
use crate::app::{App, AppErrorWith};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_units)
        .service(add_unit)
        .service(get_unit)
        .service(replace_unit)
        .service(delete_unit);
}

#[get("/units")]
async fn get_all_units(app: web::Data<App>) -> impl Responder {
    let units: Vec<models::UnitOut> = match app.get_all_units().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), units)
}

#[post("/units")]
async fn add_unit(
    new_unit: web::Json<models::UnitIn>,
    app: web::Data<App>,
    _user: User,
) -> impl Responder {
    debug!("{:?}", new_unit);

    let mut t: crate::models::NewUnit = new_unit.into_inner().into();

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
    let unit: models::UnitOut = match app.get_unit(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), unit)
}

#[put("/units/{id}")]
async fn replace_unit(
    id: web::Path<i32>,
    new_unit: web::Json<models::UnitIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_unit);

    let mut t: crate::models::NewUnit = new_unit.into_inner().into();
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
