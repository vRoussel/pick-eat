use crate::{
    api::{
        handlers::set_and_log_json_body,
        models::{self, BundleOut, PublicAccountDataOut},
    },
    app::App,
};
use actix_web::{get, web, HttpResponse, Responder};
use log::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_bundle);
}

#[get("/bundle")]
async fn get_bundle(app: web::Data<App>) -> impl Responder {
    let tags: Vec<models::TagOut> = match app.get_all_tags().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let categories: Vec<models::CategoryOut> = match app.get_all_categories().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let ingredients: Vec<models::IngredientOut> = match app.get_all_ingredients().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let units: Vec<models::UnitOut> = match app.get_all_units().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let seasons: Vec<models::SeasonOut> = match app.get_all_seasons().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let diets: Vec<models::DietOut> = match app.get_all_diets().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let accounts_with_recipes: Vec<PublicAccountDataOut> =
        match app.get_all_recipe_publishers().await {
            Ok(v) => v.into_iter().map(|x| x.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return e.into();
            }
        };

    let bundle = BundleOut {
        tags,
        categories,
        ingredients,
        units,
        seasons,
        diets,
        accounts_with_recipes,
    };
    set_and_log_json_body(HttpResponse::Ok(), bundle)
}
