use std::convert::TryInto;

use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;

use crate::api::handlers::set_and_log_json_body;
use crate::api::query_params::{RangeQueryParams, RecipeFiltersQueryParams, SortMethodQueryParams};
use crate::api::{models, Admin, User};
use crate::app::{App, AppErrorWith};
use crate::models::{Range, RangeError, RecipeFilters, SortMethod};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_recipe)
        .service(add_recipe)
        .service(get_many_recipes)
        .service(replace_recipe)
        .service(delete_recipe);
}

#[get("/recipes")]
async fn get_many_recipes(
    app: web::Data<App>,
    range_qp: web::Query<RangeQueryParams>,
    recipe_filters_qp: web::Query<RecipeFiltersQueryParams>,
    sort_method_qp: web::Query<SortMethodQueryParams>,
    user: Option<User>,
) -> impl Responder {
    let range: Range = match range_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let mut filters: RecipeFilters = match recipe_filters_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let sort_method: SortMethod = match sort_method_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let accept_range = format!("recipe {}", app.max_recipe_fetch_size());

    if let Err(e) = app.validate_recipe_range(&range) {
        let content_range = format!("{}-{}/{}", 0, 0, app.get_recipe_count());
        let mut ret = match e {
            RangeError::OutOfBounds(..) => HttpResponse::NoContent(),
            RangeError::TooWide(..) => HttpResponse::BadRequest(),
            RangeError::FromGreaterThanTo(..) => HttpResponse::BadRequest(),
            RangeError::NotStrictlyPositive(..) => HttpResponse::BadRequest(),
        };

        return ret
            .insert_header((http::header::CONTENT_RANGE, content_range))
            .insert_header((http::header::ACCEPT_RANGES, accept_range))
            .finish();
    }

    let (recipes, total_count_filtered): (Vec<models::RecipeSummaryOut>, i64) = match app
        .get_many_recipes(&range, &mut filters, sort_method, user.map(|u| u.id))
        .await
    {
        Ok(v) => {
            let t = v.get(0).map(|r| r.total_count).unwrap_or(0);
            (v.into_iter().map(|x| x.into()).collect(), t)
        }
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let fetched_count = recipes.len() as i64;
    let (first_fetched, last_fetched) = match fetched_count {
        0 => (0, 0),
        _ => (range.from, range.from + fetched_count - 1),
    };

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

    ret.insert_header((http::header::CONTENT_RANGE, content_range))
        .insert_header((http::header::ACCEPT_RANGES, accept_range));
    set_and_log_json_body(ret, recipes)
}

#[post("/recipes")]
async fn add_recipe(
    new_recipe: web::Json<models::RecipeIn>,
    app: web::Data<App>,
    user: User,
) -> impl Responder {
    debug!("{:?}", new_recipe);

    let mut t: crate::models::NewRecipe = new_recipe.into_inner().into();

    let new_id = match app.add_recipe(&mut t, user.id).await {
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

#[get("/recipes/{id}")]
async fn get_recipe(id: web::Path<i32>, app: web::Data<App>, user: Option<User>) -> impl Responder {
    let recipe: models::RecipeOut = match app.get_recipe(id.into_inner(), user.map(|u| u.id)).await
    {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), recipe)
}

#[put("/recipes/{id}")]
async fn replace_recipe(
    id: web::Path<i32>,
    new_recipe: web::Json<models::RecipeIn>,
    app: web::Data<App>,
    user: User,
) -> impl Responder {
    debug!("{:?}", new_recipe);

    let mut t: crate::models::NewRecipe = new_recipe.into_inner().into();
    match app
        .replace_recipe(id.into_inner(), &mut t, user.id, user.is_admin)
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            use AppErrorWith::*;
            if let AppError(inner_e) = &e {
                error!("{}", inner_e);
            }
            return e.into();
        }
    };

    HttpResponse::Ok().finish()
}

#[delete("/recipes/{id}")]
async fn delete_recipe(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_recipe(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
