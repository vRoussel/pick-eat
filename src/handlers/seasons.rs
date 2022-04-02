use crate::database::Pool;
use crate::resources::season;
use actix_web::{get, web, HttpResponse, Responder};
use log::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all).service(get_one);
}

#[get("/seasons")]
pub async fn get_all(db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let seasonss = match season::get_many(&db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&seasonss).unwrap());
    HttpResponse::Ok().json(seasonss)
}

#[get("/seasons/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let season = match season::get_one(&db_conn, id.into_inner()).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&season).unwrap());
    HttpResponse::Ok().json(season)
}
