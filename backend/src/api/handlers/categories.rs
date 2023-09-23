use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::{
    api::{models, Admin},
    app::{App, AppErrorWith},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_categories)
        .service(add_category)
        .service(get_category)
        .service(replace_category)
        .service(delete_category);
}

#[get("/categories")]
async fn get_all_categories(app: web::Data<App>) -> impl Responder {
    let categories: Vec<models::CategoryOut> = match app.get_all_categories().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&categories) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(categories)
}

#[post("/categories")]
async fn add_category(
    new_category: web::Json<models::CategoryIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_category);

    let mut t: crate::models::NewCategory = new_category.into_inner().into();

    let new_id = match app.add_category(&mut t).await {
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

#[get("/categories/{id}")]
async fn get_category(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let category: models::CategoryOut = match app.get_category(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&category) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(category)
}

#[put("/categories/{id}")]
async fn replace_category(
    id: web::Path<i32>,
    new_category: web::Json<models::CategoryIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_category);

    let mut t: crate::models::NewCategory = new_category.into_inner().into();
    match app.replace_category(id.into_inner(), &mut t).await {
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

#[delete("/categories/{id}")]
async fn delete_category(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_category(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
