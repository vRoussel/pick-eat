use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::{Deserialize, Serialize};

use crate::api::{Admin, User};
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidAccountUpdate, InvalidNewAccount};

use super::APIError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(add_account)
        .service(delete_account)
        .service(delete_current_account)
        .service(get_current_account)
        .service(get_all_public_accounts_data)
        .service(replace_current_account)
        .service(add_recipe_to_account_favs)
        .service(remove_recipe_from_account_favs);
}

fn display_name_invalidity_to_api_error(
    display_name_inv: models::InvalidityKind,
) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match display_name_inv {
        Kind::AlreadyUsed => Some(APIError {
            message: "Ce pseudo est déjà pris",
            field: Some("display_name"),
            code: None,
        }),
        Kind::Empty => Some(APIError {
            message: "Un pseudo ne peut pas être vide",
            field: Some("display_name"),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's display_name, this should not happen",
                display_name_inv
            );
            None
        }
    }
}

fn email_invalidity_to_api_error(email_inv: models::InvalidityKind) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match email_inv {
        Kind::AlreadyUsed => Some(APIError {
            message: "Cet email est déjà utilisé",
            field: Some("email"),
            code: None,
        }),
        Kind::Empty => Some(APIError {
            message: "Veuillez saisir une adresse mail",
            field: Some("email"),
            code: None,
        }),
        Kind::BadFormat => Some(APIError {
            message: "Cet email est invalide",
            field: Some("email"),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's email, this should not happen",
                email_inv
            );
            None
        }
    }
}

fn password_invalidity_to_api_error(
    password_inv: models::InvalidityKind,
    field_name: &'static str,
) -> Option<APIError> {
    type Kind = models::InvalidityKind;
    match password_inv {
        Kind::TooShort => Some(APIError {
            message: "Le mot de passe doit contenir au moins 8 caractères",
            field: Some(field_name),
            code: None,
        }),
        _ => {
            warn!(
                "{:?} error received for account's {}, this should not happen",
                password_inv, field_name
            );
            None
        }
    }
}

impl From<InvalidNewAccount> for Vec<APIError> {
    fn from(value: InvalidNewAccount) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.display_name {
            display_name_invalidity_to_api_error(v).map(|e| ret.push(e));
        };
        if let Some(v) = value.email {
            email_invalidity_to_api_error(v).map(|e| ret.push(e));
        }
        if let Some(v) = value.password {
            password_invalidity_to_api_error(v, "password").map(|e| ret.push(e));
        }
        ret
    }
}

impl From<InvalidAccountUpdate> for Vec<APIError> {
    fn from(value: InvalidAccountUpdate) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.display_name {
            display_name_invalidity_to_api_error(v).map(|e| ret.push(e));
        };
        if let Some(v) = value.email {
            email_invalidity_to_api_error(v).map(|e| ret.push(e));
        }
        if let Some(v) = value.new_password {
            password_invalidity_to_api_error(v, "new_password").map(|e| ret.push(e));
        }
        if let Some(v) = value.old_password {
            match v {
                models::InvalidityKind::Empty => {
                    ret.push(APIError {
                        message: "Le mot de passe actuel doit être renseigné",
                        field: Some("old_password"),
                        code: None,
                    });
                }
                models::InvalidityKind::Mismatch => {
                    ret.push(APIError {
                        message: "Le mot de passe est incorrect",
                        field: Some("old_password"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for accountupdate's old password, this should not happen",
                        v
                    );
                }
            };
        }
        ret
    }
}

#[derive(Debug, Serialize)]
struct AccountOut {
    id: i32,
    display_name: String,
    email: String,
    creation_date: chrono::NaiveDate,
    is_admin: bool,
    is_validated: bool,
}

#[derive(Debug, Serialize)]
struct PublicAccountDataOut {
    id: i32,
    display_name: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NewAccountIn {
    display_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AccountUpdateIn {
    display_name: String,
    email: String,
    old_password: String,
    new_password: Option<String>,
}

impl std::fmt::Debug for NewAccountIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NewAccountIn {{ display_name: {}, email: <hidden>, password: <hidden> }}",
            self.display_name
        )
    }
}

impl std::fmt::Debug for AccountUpdateIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AccountUpdateIn {{ display_name: {}, email: <hidden>, old_password: <hidden>, new_password: <hidden> }}",
            self.display_name
        )
    }
}

impl From<NewAccountIn> for models::NewAccount {
    fn from(a: NewAccountIn) -> Self {
        Self {
            display_name: a.display_name,
            email: a.email,
            password: models::Password::ClearText(a.password),
        }
    }
}

impl From<AccountUpdateIn> for models::AccountUpdate {
    fn from(a: AccountUpdateIn) -> Self {
        Self {
            display_name: a.display_name,
            email: a.email,
            old_password: models::Password::ClearText(a.old_password),
            new_password: a.new_password.map(|v| models::Password::ClearText(v)),
        }
    }
}

impl From<models::Account> for AccountOut {
    fn from(a: models::Account) -> Self {
        Self {
            id: a.id,
            display_name: a.display_name,
            email: a.email,
            creation_date: a.creation_date,
            is_admin: a.is_admin,
            is_validated: a.is_validated,
        }
    }
}

impl From<models::PublicAccountData> for PublicAccountDataOut {
    fn from(a: models::PublicAccountData) -> Self {
        Self {
            id: a.id,
            display_name: a.display_name,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetAllQueryParams {
    withrecipes: bool,
}

#[get("/accounts")]
pub async fn get_all_public_accounts_data(
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

    match serde_json::to_string(&accounts) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(accounts)
}

#[post("/accounts")]
pub async fn add_account(
    new_account: web::Json<NewAccountIn>,
    app: web::Data<App>,
) -> impl Responder {
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
pub async fn get_current_account(user: User, app: web::Data<App>) -> impl Responder {
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

    match serde_json::to_string(&account) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(account)
}

#[put("/accounts/me")]
pub async fn replace_current_account(
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
pub async fn delete_account(
    id: web::Path<i32>,
    app: web::Data<App>,
    _admin: Admin,
) -> impl Responder {
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
pub async fn delete_current_account(user: User, app: web::Data<App>) -> impl Responder {
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
pub async fn add_recipe_to_account_favs(
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
pub async fn remove_recipe_from_account_favs(
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
