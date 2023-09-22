use crate::{app::App, models};
use actix_web::{get, web, HttpResponse, Responder};
use log::*;
use serde::Serialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_seasons).service(get_season);
}

#[derive(Debug, Serialize)]
pub struct SeasonOut {
    id: i32,
    name: String,
    label: String,
}

impl From<models::Season> for SeasonOut {
    fn from(s: models::Season) -> Self {
        Self {
            id: s.id,
            name: s.name,
            label: s.label,
        }
    }
}

#[get("/seasons")]
async fn get_all_seasons(app: web::Data<App>) -> impl Responder {
    let seasons: Vec<SeasonOut> = match app.get_all_seasons().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&seasons) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(seasons)
}

#[get("/seasons/{id}")]
async fn get_season(id: web::Path<i32>, app: web::Data<App>) -> impl Responder {
    let season: SeasonOut = match app.get_season(id.into_inner()).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&season) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(season)
}
