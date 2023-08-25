use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::{Error, PgConnection};

use super::{APIError, Admin};
use crate::api::APIAnswer;
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidTag};
use crate::storage;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tags)
        .service(add_tag)
        .service(get_tag)
        .service(replace_tag)
        .service(delete_tag);
}

impl From<InvalidTag> for Vec<APIError> {
    fn from(value: InvalidTag) -> Self {
        type kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                kind::AlreadyUsed => {
                    ret.push(APIError {
                        message: "Un tag avec ce nom existe déjà",
                        field: Some("name"),
                        code: None,
                    });
                }
            };
        };
        ret
    }
}

#[derive(Debug, Serialize)]
struct TagOut {
    id: i32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct TagIn {
    name: String,
}

impl From<TagIn> for models::NewTag {
    fn from(t: TagIn) -> Self {
        Self { name: t.name }
    }
}

impl From<models::Tag> for TagOut {
    fn from(t: models::Tag) -> Self {
        Self {
            id: t.id,
            name: t.name,
        }
    }
}

#[get("/tags")]
pub async fn get_all_tags(app: web::Data<App>) -> impl Responder {
    let tags: Vec<TagOut> = match app.get_all_tags().await {
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
pub async fn add_tag(
    new_tag: web::Json<TagIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_tag);

    let mut t: models::NewTag = new_tag.into_inner().into();

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
pub async fn get_tag(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let tag: TagOut = match app.get_tag(id.into_inner()).await {
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
pub async fn replace_tag(
    id: web::Path<i32>,
    new_tag: web::Json<TagIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_tag);

    let mut t: models::NewTag = new_tag.into_inner().into();
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
pub async fn delete_tag(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
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
