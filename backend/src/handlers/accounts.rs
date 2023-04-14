use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::Deserialize;
use sqlx::postgres::PgPool;
use sqlx::PgConnection;

use crate::handlers::{APIAnswer, Admin, User};
use crate::resources::account;
use crate::resources::token::{
    check_validation_token_validity, delete_all_account_validation_tokens, TokenValidity,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_one)
        .service(delete_one)
        .service(delete_me)
        .service(get_current)
        .service(get_all)
        .service(modify_current)
        .service(add_fav_recipe)
        .service(remove_fav_recipe)
        .service(validate_account);
}

async fn validate_new_account(
    account: &account::New,
    api_answer: &mut APIAnswer,
    db_conn: &mut PgConnection,
) {
    if let Ok(Some(_)) = account::get_one_by_email(db_conn, &account.email).await {
        api_answer.add_field_error("email", "Cette addresse email est déjà utilisée");
    }
    if let Ok(Some(_)) = account::get_one_by_name(db_conn, &account.display_name).await {
        api_answer.add_field_error("name", "Ce nom est déjà utilisé");
    }
    if account.password.len() < 8 {
        api_answer.add_field_error(
            "new_password",
            "Le mot de passe doit faire au moins 8 caractères",
        );
    }
}

#[post("/accounts")]
pub async fn add_one(
    account: web::Json<account::New>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let mut ret = APIAnswer::new();
    validate_new_account(&account, &mut ret, &mut db_conn).await;
    if !ret.is_ok() {
        return HttpResponse::BadRequest().json(ret);
    }

    let new_id = match account::add_one(&mut db_conn, &account).await {
        Ok(v) => v,
        Err(e) => match e {
            _ => {
                error!("{:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[delete("/accounts/{id}")]
pub async fn delete_one(
    id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
    _admin: Admin,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match account::delete_one(&mut db_conn, id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}

#[delete("/accounts/me")]
pub async fn delete_me(user: User, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match account::delete_one(&mut db_conn, user.id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::NoContent().finish()
}

#[get("/accounts/me")]
pub async fn get_current(user: User, db_pool: web::Data<PgPool>) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let account = match account::get_one(&mut db_conn, user.id).await {
        Ok(Some(v)) => v,
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };
    HttpResponse::Ok().json(account)
}

#[derive(Debug, Deserialize)]
pub struct GetAllQueryParams {
    withrecipes: bool,
}

#[get("/accounts")]
pub async fn get_all(
    params: web::Query<GetAllQueryParams>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();
    if !params.withrecipes {
        return HttpResponse::BadRequest().finish();
    }

    let accounts = match account::get_all_with_recipes(&mut db_conn).await {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    trace!("{}", serde_json::to_string_pretty(&accounts).unwrap());
    HttpResponse::Ok().json(accounts)
}

async fn validate_update_account(
    current: &account::FromDBPrivate,
    update: &account::Update,
    api_answer: &mut APIAnswer,
    db_conn: &mut PgConnection,
) {
    if let Err(_) = account::check_password(db_conn, current.id, &update.old_password).await {
        api_answer.add_field_error("old_password", "Mot de passe incorrect");
    };
    if current.email != update.new_email {
        if let Ok(Some(_)) = account::get_one_by_email(db_conn, &update.new_email).await {
            api_answer.add_field_error("new_email", "Cette addresse email est déjà utilisée");
        }
    }
    if current.display_name != update.new_display_name {
        if let Ok(Some(_)) = account::get_one_by_name(db_conn, &update.new_display_name).await {
            api_answer.add_field_error("new_name", "Ce nom est déjà utilisé");
        }
    }

    if let Some(pwd) = &update.new_password {
        if pwd.len() < 8 {
            api_answer.add_field_error(
                "new_password",
                "Le mot de passe doit faire au moins 8 caractères",
            );
        }
    }
}

#[put("/accounts/me")]
pub async fn modify_current(
    account: web::Json<account::Update>,
    user: User,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    let existing_account = match account::get_one(&mut db_conn, user.id).await {
        Ok(Some(v)) => v,
        _ => return HttpResponse::InternalServerError().finish(),
    };

    let mut ret = APIAnswer::new();
    validate_update_account(&existing_account, &account, &mut ret, &mut db_conn).await;
    if !ret.is_ok() {
        return HttpResponse::BadRequest().json(ret);
    }

    match account::modify_one(&mut db_conn, user.id, &account).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => match e {
            _ => {
                error!("{:?}", e);
                return HttpResponse::InternalServerError().finish();
            }
        },
    };

    HttpResponse::Ok().finish()
}

#[put("/accounts/me/favorites/{recipe_id}")]
pub async fn add_fav_recipe(
    user: User,
    recipe_id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match account::add_fav_recipe(&mut db_conn, user.id, *recipe_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[delete("/accounts/me/favorites/{recipe_id}")]
pub async fn remove_fav_recipe(
    user: User,
    recipe_id: web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut db_conn = db_pool.acquire().await.unwrap();

    match account::remove_fav_recipe(&mut db_conn, user.id, *recipe_id).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    }
    HttpResponse::Ok().finish()
}

#[post("/accounts/validation")]
pub async fn validate_account(
    req: web::Json<account::Validate>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let mut transaction = db_pool.begin().await.unwrap();
    let validity = check_validation_token_validity(&mut transaction, &req.token).await;
    let account_id = match validity {
        Ok(v) => match v {
            TokenValidity::Valid { account_id } => account_id,
            _ => {
                let mut ret = APIAnswer::new();
                ret.add_error("Ce lien est incorrect ou expiré, essayez de vous connecter pour en générer un nouveau");
                return HttpResponse::Unauthorized().json(ret);
            }
        },
        Err(e) => {
            error!("{}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    if account::validate_account(&mut transaction, account_id)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if delete_all_account_validation_tokens(&mut transaction, account_id)
        .await
        .is_err()
    {
        return HttpResponse::InternalServerError().finish();
    }

    if transaction.commit().await.is_err() {
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
