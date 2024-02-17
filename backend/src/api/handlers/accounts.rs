use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;

use crate::api::handlers::set_and_log_json_body;
use crate::api::models::{AccountOut, AccountUpdateIn, NewAccountIn, PublicAccountDataOut};
use crate::api::{Admin, User};
use crate::app::{App, AppErrorWith};
use crate::models;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_account)
        .service(delete_account)
        .service(delete_current_account)
        .service(get_current_account)
        .service(get_all_public_accounts_data)
        .service(replace_current_account)
        .service(add_recipe_to_account_favs)
        .service(remove_recipe_from_account_favs)
        .service(save_account_data)
        .service(get_account_data);
}

#[derive(Debug, Deserialize)]
struct GetAllQueryParams {
    withrecipes: bool,
}

#[get("/accounts")]
async fn get_all_public_accounts_data(
    app: web::Data<App>,
    params: web::Query<GetAllQueryParams>,
) -> impl Responder {
    if !params.withrecipes {
        return HttpResponse::BadRequest().finish();
    }

    let accounts: Vec<PublicAccountDataOut> = match app.get_all_recipe_publishers().await {
        Ok(v) => v.into_iter().map(|x| x.into()).collect(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), accounts)
}

#[post("/accounts")]
async fn add_account(new_account: web::Json<NewAccountIn>, app: web::Data<App>) -> impl Responder {
    debug!("{:?}", new_account);

    let mut a: models::NewAccount = new_account.into_inner().into();

    let new_id = match app.add_account(&mut a).await {
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

#[get("/accounts/me")]
async fn get_current_account(user: User, app: web::Data<App>) -> impl Responder {
    let account: AccountOut = match app.get_account_by_id(user.id).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    set_and_log_json_body(HttpResponse::Ok(), account)
}

#[put("/accounts/me")]
async fn replace_current_account(
    account: web::Json<AccountUpdateIn>,
    app: web::Data<App>,
    user: User,
) -> impl Responder {
    debug!("{:?}", account);

    let mut a: models::AccountUpdate = account.into_inner().into();
    match app.replace_account(user.id, &mut a).await {
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

#[delete("/accounts/{id}")]
async fn delete_account(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_account(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
    HttpResponse::NoContent().finish()
}

#[delete("/accounts/me")]
async fn delete_current_account(user: User, app: web::Data<App>) -> impl Responder {
    match app.delete_account(user.id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
    HttpResponse::NoContent().finish()
}

#[put("/accounts/me/favorites/{recipe_id}")]
async fn add_recipe_to_account_favs(
    user: User,
    recipe_id: web::Path<i32>,
    app: web::Data<App>,
) -> impl Responder {
    match app.add_recipe_to_account_favs(user.id, *recipe_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
    HttpResponse::Ok().finish()
}

#[delete("/accounts/me/favorites/{recipe_id}")]
async fn remove_recipe_from_account_favs(
    user: User,
    recipe_id: web::Path<i32>,
    app: web::Data<App>,
) -> impl Responder {
    match app
        .remove_recipe_from_account_favs(user.id, *recipe_id)
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
    HttpResponse::Ok().finish()
}

#[put("/accounts/me/data/{key}")]
async fn save_account_data(
    user: User,
    key: web::Path<String>,
    data: web::Json<serde_json::Value>,
    app: web::Data<App>,
) -> impl Responder {
    match app
        .save_account_data(user.id, &*key, data.into_inner())
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    }
    HttpResponse::Ok().finish()
}

#[get("/accounts/me/data/{key}")]
async fn get_account_data(
    user: User,
    key: web::Path<String>,
    app: web::Data<App>,
) -> impl Responder {
    let data: serde_json::Value = match app.get_account_data(user.id, &*key).await {
        Ok(Some(v)) => v,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };
    set_and_log_json_body(HttpResponse::Ok(), data)
}
