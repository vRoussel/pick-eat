use actix_web::{get, post, put, delete, web, Responder, http};
use crate::database::Pool;
use log::*;
use tokio_postgres::{error::SqlState};

use crate::resources::ingredient;
use crate::utils::*;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
       .service(add_one)
       .service(get_one)
       .service(modify_one)
       .service(delete_one)
    ;
}

#[get("/ingredients")]
pub async fn get_all(params: web::Query<HttpParams>, db_pool: web::Data<Pool>) -> impl Responder {
    let (min, max) = params.range();
    let db_conn = db_pool.get().await.unwrap();
    let ingredients_query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            u.id as default_unit_id, \
            u.full_name as default_unit_full_name, \
            u.short_name as default_unit_short_name \
        FROM
            ingredients as i \
            LEFT JOIN units as u \
            ON i.default_unit_id = u.id \
        ORDER BY name
        OFFSET $1
        LIMIT $2
    ";

    let ingredients: Vec<ingredient::FromDB> = match db_conn.query(ingredients_query, &[&(min-1), &(max-min+1)])
        .await {
            Ok(rows) => rows.iter().map(|r| r.into()).collect(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let count_query = "SELECT count(*) FROM ingredients";
    let n_ingredients: usize = match db_conn.query(count_query, &[])
        .await {
            Ok(rows) => rows[0].get::<usize,i64>(0) as usize,
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
    };

    let mut ret = web::HttpResponse::PartialContent();
    if n_ingredients == ingredients.len() {
        ret = web::HttpResponse::Ok();
    }

    ret.body(format!("{}", serde_json::to_string_pretty(&ingredients).unwrap()))
}

#[post("/ingredients")]
pub async fn add_one(new_ingredient: web::Json<ingredient::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    trace!("{:#?}", new_ingredient);
    let insert_query = "\
        INSERT INTO ingredients (name, default_unit_id) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    let new_id = match db_conn.query(insert_query, &[&new_ingredient.name, &new_ingredient.default_unit_id])
        .await {
            Ok(rows) => rows[0].get::<&str,i32>("id").to_string(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                //TODO add location with URI
                => return web::HttpResponse::Conflict().finish(),
            Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
                => return web::HttpResponse::UnprocessableEntity().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish();
            }
        };
    web::HttpResponse::Created()
        .set_header(http::header::LOCATION, format!("/{}", new_id))
        .finish()

}

#[get("/ingredients/{id}")]
pub async fn get_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            u.id as default_unit_id, \
            u.full_name as default_unit_full_name, \
            u.short_name as default_unit_short_name \
        FROM
            ingredients as i \
            LEFT JOIN units as u \
            ON i.default_unit_id = u.id \
        WHERE
            i.id = $1 \
    ";

    let ingredient: ingredient::FromDB = match db_conn.query(query, &[&id])
        .await {
            Ok(rows) if rows.len() == 1 => (&rows[0]).into(),
            Ok(rows) if rows.len() == 0 => return web::HttpResponse::NotFound().finish(),
            Err(e) => {error!("{}", e); return web::HttpResponse::InternalServerError().finish()},
            _ => return web::HttpResponse::InternalServerError().finish(),
    };
    web::HttpResponse::Ok().body(format!("{}", serde_json::to_string_pretty(&ingredient).unwrap()))

}

#[put("/ingredients/{id}")]
pub async fn modify_one(id: web::Path<i32>, new_ingredient: web::Json<ingredient::New>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    trace!("{:#?}", new_ingredient);
    let update_query = "\
        UPDATE ingredients SET \
            name = $1, \
            default_unit_id = $2 \
        WHERE id = $3 \
        RETURNING id;
    ";
    match db_conn.query(update_query,
        &[&new_ingredient.name, &new_ingredient.default_unit_id, &id])
        .await {
            Ok(rows) if rows.len() == 0
                => return web::HttpResponse::NotFound().finish(),
            Err(ref e) if e.code() == Some(&SqlState::UNIQUE_VIOLATION)
                => return web::HttpResponse::Conflict().finish(),
            Err(ref e) if e.code() == Some(&SqlState::FOREIGN_KEY_VIOLATION)
                => return web::HttpResponse::UnprocessableEntity().finish(),
            Err(e) => {
                error!("{}", e);
                return web::HttpResponse::InternalServerError().finish()
            },
            Ok(_) => ()
        };
    web::HttpResponse::Ok().finish()
}

#[delete("/ingredients/{id}")]
pub async fn delete_one(id: web::Path<i32>, db_pool: web::Data<Pool>) -> impl Responder {
    let db_conn = db_pool.get().await.unwrap();
    let id = id.into_inner();
    let delete_query = "\
        DELETE FROM ingredients \
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
