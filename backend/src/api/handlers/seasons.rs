use crate::{
    api::{handlers::set_and_log_json_body, models},
    app::App,
};
use actix_web::{get, web, HttpResponse, Responder};
use log::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_seasons).service(get_season);
}

#[get("/seasons")]
async fn get_all_seasons(app: web::Data<App>) -> impl Responder {
    let seasons: Vec<models::SeasonOut> = match app.get_all_seasons().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), seasons)
}

#[get("/seasons/{id}")]
async fn get_season(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let season: models::SeasonOut = match app.get_season(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), season)
}
