use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::{
    api::{models, Admin, User},
    app::{App, AppErrorWith},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_ingredients)
        .service(add_ingredient)
        .service(get_ingredient)
        .service(replace_ingredient)
        .service(delete_ingredient);
}

#[get("/ingredients")]
async fn get_all_ingredients(app: web::Data<App>) -> impl Responder {
    let ingredients: Vec<models::IngredientOut> = match app.get_all_ingredients().await {
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
    new_ingredient: web::Json<models::IngredientIn>,
    app: web::Data<App>,
    _user: User,
) -> impl Responder {
    debug!("{:?}", new_ingredient);

    let mut i: crate::models::NewIngredient = new_ingredient.into_inner().into();

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
    let ingredient: models::IngredientOut = match app.get_ingredient(id.into_inner()).await {
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
    new_ingredient: web::Json<models::IngredientIn>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
    debug!("{:?}", new_ingredient);

    let mut i: crate::models::NewIngredient = new_ingredient.into_inner().into();
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
