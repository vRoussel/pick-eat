use super::db_error_to_http_response;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;
use serde_json;
use sqlx::postgres::PgPool;
use sqlx::Error;

use crate::handlers::{Admin, User};
use crate::query_params::{Range, RangeError};
use crate::resources::recipe::{self, Filter};
use crate::resources::{get_total_count, isolate_transaction};

static MAX_PER_REQUEST: i64 = 50;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all)
        .service(add_one)
        .service(get_one)
        .service(modify_one)
        .service(delete_one);
}

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    range: Range,
    search: Option<String>,
    categories: Option<String>,
    seasons: Option<String>,
    ingredients: Option<String>,
    tags: Option<String>,
    account: Option<i32>,
    diets: Option<String>,
    sort: String,
    seed: Option<i32>,
}

#[get("/recipes")]
pub async fn get_all(
    params: web::Query<GetQueryParams>,
    db_pool: web::Data<PgPool>,
    user: Option<User>,
) -> impl Responder {
    let mut db_conn = db_pool.begin().await.unwrap();

    // Make sure we ignore changes that occur between our
    // 2 requests (get_total_count and get_recipes)
    if let Err(e) = isolate_transaction(&mut db_conn).await {
        error!("{}", e);
        return HttpResponse::InternalServerError().finish();
    }
    let total_count: i64 = match get_total_count(&mut db_conn, "recipes").await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let accept_range = format!("recipe {}", MAX_PER_REQUEST);

    if let Err(e) = params
        .range
        .validate(Some(MAX_PER_REQUEST), Some(total_count))
    {
        let content_range = format!("{}-{}/{}", 0, 0, total_count);
        let mut ret = match e {
            RangeError::OutOfBounds => HttpResponse::NoContent(),
            RangeError::TooWide => HttpResponse::BadRequest(),
            RangeError::Invalid => HttpResponse::BadRequest(),
        };

        return ret
            .insert_header((http::header::CONTENT_RANGE, content_range))
            .insert_header((http::header::ACCEPT_RANGES, accept_range))
            .finish();
    }

    let sort_method = match params.sort.as_str() {
        "random" => match params.seed {
            Some(s) => recipe::SortMethod::Random { seed: s },
            _ => return HttpResponse::BadRequest().finish(),
        },
        _ => return HttpResponse::BadRequest().finish(),
    };

    let mut filters: Vec<Filter> = Vec::new();
    if let Some(val) = &params.search {
        filters.push(Filter::Search(val.to_string()));
    }
    if let Some(val) = &params.categories {
        filters.push(Filter::Categories(
            val.split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    if let Some(val) = &params.seasons {
        filters.push(Filter::Seasons(
            val.split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    if let Some(val) = &params.ingredients {
        filters.push(Filter::Ingredients(
            val.split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    if let Some(val) = &params.tags {
        filters.push(Filter::Tags(
            val.split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }
    if let Some(val) = &params.account {
        filters.push(Filter::Account(*val));
    }
    if let Some(val) = &params.diets {
        filters.push(Filter::Diets(
            val.split(",").map(|x| x.parse().unwrap()).collect(),
        ));
    }

    let (recipes, total_count_filtered) = match recipe::get_many(
        &mut db_conn,
        &params.range,
        &filters,
        &sort_method,
        user.map(|u| u.id),
    )
    .await
    {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    if let Err(e) = db_conn.rollback().await {
        error!("{}", e);
        return HttpResponse::InternalServerError().finish();
    }

    let fetched_count = recipes.len() as i64;
    let (first_fetched, last_fetched);

    if fetched_count == 0 {
        first_fetched = 0;
        last_fetched = 0;
    } else {
        first_fetched = params.range.from;
        last_fetched = first_fetched + fetched_count - 1;
    }

    let mut ret;
    if fetched_count < total_count_filtered && fetched_count > 0 {
        ret = HttpResponse::PartialContent();
    } else {
        ret = HttpResponse::Ok();
    }
    let content_range = format!(
        "{}-{}/{}",
        first_fetched, last_fetched, total_count_filtered
    );

    trace!("{}", serde_json::to_string_pretty(&recipes).unwrap());
    ret.insert_header((http::header::CONTENT_RANGE, content_range))
        .insert_header((http::header::ACCEPT_RANGES, accept_range))
        .json(recipes)
}

#[post("/recipes")]
pub async fn add_one(
    new_recipe: web::Json<recipe::New>,
    db_pool: web::Data<PgPool>,
    user: User,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    trace!("{:#?}", new_recipe);
    let new_id = match recipe::add_one(&mut db_conn, &new_recipe, user.id).await {
        Ok(v) => v,
        Err(e) => match e {
            Error::Database(db_error) => {
                error!("{}", db_error);
                return db_error_to_http_response(&*db_error);
            }
            _ => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[get("/recipes/{id}")]
pub async fn get_one(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
    user: Option<User>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let recipe = match recipe::get_one(&mut db_conn, id.into_inner(), user.map(|u| u.id)).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&recipe).unwrap());
    HttpResponse::Ok().json(recipe)
}

#[put("/recipes/{id}")]
pub async fn modify_one(
    id: web::Path<i32>,
    new_recipe: web::Json<recipe::New>,
    db_pool: web::Data<PgPool>,
    user: User,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    let recipe_id = id.into_inner();
    let Ok(recipe_author_id) = recipe::get_author_id(&mut db_conn, recipe_id).await else {
        return HttpResponse::InternalServerError().finish();
    };

    if !(user.is_admin || recipe_author_id == user.id) {
        return HttpResponse::Forbidden().finish();
    }

    trace!("{:#?}", new_recipe);

    match recipe::modify_one(&mut db_conn, recipe_id, &new_recipe).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => match e {
            Error::Database(db_error) => {
                error!("{}", db_error);
                return db_error_to_http_response(&*db_error);
            }
            _ => {
                error!("{}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    }
    HttpResponse::Ok().finish()
}

#[delete("/recipes/{id}")]
pub async fn delete_one(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
    _admin: Admin,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match recipe::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}
