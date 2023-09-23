use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::{
    api::{models, Admin},
    app::{App, AppErrorWith},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tags)
        .service(add_tag)
        .service(get_tag)
        .service(replace_tag)
        .service(delete_tag);
}

#[get("/tags")]
async fn get_all_tags(app: web::Data<App>) -> impl Responder {
    let tags: Vec<models::TagOut> = match app.get_all_tags().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&tags) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(tags)
}

#[post("/tags")]
async fn add_tag(
    new_tag: web::Json<models::TagIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_tag);

    let mut t: crate::models::NewTag = new_tag.into_inner().into();

    let new_id = match app.add_tag(&mut t).await {
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

#[get("/tags/{id}")]
async fn get_tag(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let tag: models::TagOut = match app.get_tag(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&tag) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(tag)
}

#[put("/tags/{id}")]
async fn replace_tag(
    id: web::Path<i32>,
    new_tag: web::Json<models::TagIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_tag);

    let mut t: crate::models::NewTag = new_tag.into_inner().into();
    match app.replace_tag(id.into_inner(), &mut t).await {
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

#[delete("/tags/{id}")]
async fn delete_tag(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_tag(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
