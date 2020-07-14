use actix_web::{get, post, put, delete, web, Responder, http};
use crate::database::Pool;
use log::*;
use tokio_postgres::error::SqlState;

use crate::resources::unit;
use crate::utils::*;

static MAX_PER_REQUEST: i64 = 100;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_one)
       .service(delete_one)
    ;
}

#[get("/units")]
pub async fn get_all(params: web::Query<HttpParams>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();

    let count_query = "SELECT count(*) FROM units";
    let n_all_units: i64 = match db_conn.query(count_query, &[])
        .await {
            Ok(rows) => rows[0].get(0),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let accept_range = format!("unit {}", MAX_PER_REQUEST);

    let (min, max) = params.range();
    if max - min + 1 > MAX_PER_REQUEST {
        return web::HttpResponse::BadRequest()
            .set_header(http::header::ACCEPT_RANGES, accept_range)
            .finish()
    } else if min > n_all_units {
        let content_range = format!("{}-{}/{}", 0, 0, n_all_units);
        return web::HttpResponse::NoContent()
            .set_header(http::header::CONTENT_RANGE, content_range)
            .set_header(http::header::ACCEPT_RANGES, accept_range)
            .finish()
    }

    let units_query = "\
        SELECT \
            id, \
            full_name, \
            short_name \
        FROM units \
        ORDER BY full_name
        OFFSET $1
        LIMIT $2
    ";

    let units: Vec<unit::FromDB> = match db_conn.query(units_query, &[&(min-1), &(max-min+1)])
        .await {
            Ok(rows) => rows.iter().map(|r| r.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let n_fetched_units = units.len() as i64;

    let mut ret = web::HttpResponse::PartialContent();
    if n_all_units == n_fetched_units {
        ret = web::HttpResponse::Ok();
    }

    let content_range = format!("{}-{}/{}", min, min + n_fetched_units - 1, n_all_units);

    ret.set_header(http::header::CONTENT_RANGE, content_range)
       .set_header(http::header::ACCEPT_RANGES, accept_range)
       .body(format!("{}", serde_json::to_string_pretty(&units).unwrap()))
}

#[post("/units")]
pub async fn add_one(new_unit: web::Json<unit::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_unit);
    let insert_query = "\
        INSERT INTO units (full_name, short_name) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    let new_id = match db_conn.query(insert_query, &[&new_unit.full_name, &new_unit.short_name])
        .await {
            Ok(rows) => rows[0].get::<&str,i32>("id").to_string(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                //TODO add location with URI
                => return web::HttpResponse::Conflict().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
        };
    web::HttpResponse::Created()
        .set_header(http::header::LOCATION, format!("/{}", new_id))
        .finish()
}

#[get("/units/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let query = "\
        SELECT \
            id, \
            full_name, \
            short_name \
        FROM units \
        WHERE id = $1 \
    ";

    let unit: unit::FromDB = match db_conn.query(query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => (&rows[0]).into(),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Ok(_) => return web::HttpResponse::InternalServerError().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
    };
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&unit).unwrap()))

}

#[put("/units/{id}")]
pub async fn modify_one(id: web::Path<i32>, new_unit: web::Json<unit::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    trace!("{:#?}", new_unit);
    let update_query = "\
        UPDATE units SET \
            full_name = $1, \
            short_name = $2, \
        WHERE id = $3 \
        RETURNING id;
    ";
    match db_conn.query(update_query,
        &[&new_unit.full_name, &new_unit.short_name, &id])
        .await {
            Ok(rows) if rows.len() == 0
                => return web::HttpResponse::NotFound().finish(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                => return web::HttpResponse::Conflict().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
            Ok(_) => ()
        };
    web::HttpResponse::Ok().finish()
}

#[delete("/units/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let delete_query = "\
        DELETE FROM units \
        WHERE id = $1 \
        RETURNING id;
    ";
    match db_conn.query(delete_query, &[&id])
        .await {
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
            Ok(_) => (),
        };
    web::HttpResponse::NoContent().finish()
}
