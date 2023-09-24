use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::api::handlers::set_and_log_json_body;
use crate::api::{models, Admin};
use crate::app::{App, AppErrorWith};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_diets)
        .service(add_diet)
        .service(get_diet)
        .service(replace_diet)
        .service(delete_diet);
}

#[get("/diets")]
async fn get_all_diets(app: web::Data<App>) -> impl Responder {
    let diets: Vec<models::DietOut> = match app.get_all_diets().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), diets)
}

#[post("/diets")]
async fn add_diet(
    new_diet: web::Json<models::DietIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_diet);

    let mut t: crate::models::NewDiet = new_diet.into_inner().into();

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
    let diet: models::DietOut = match app.get_diet(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), diet)
}

#[put("/diets/{id}")]
async fn replace_diet(
    id: web::Path<i32>,
    new_diet: web::Json<models::DietIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_diet);

    let mut t: crate::models::NewDiet = new_diet.into_inner().into();
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
