use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{self, retry_up_to_n_times},
};

use email_address::EmailAddress;

fn check_new_account_input(input: &NewAccount) -> Result<(), InvalidNewAccount> {
    let mut invalid_account = InvalidNewAccount::default();
    if let Password::ClearText(p) = &input.password {
        if p.len() < 8 {
            invalid_account.password = Some(InvalidityKind::TooShort);
        }
    }
    if !EmailAddress::is_valid(&input.email) {
        invalid_account.email = Some(InvalidityKind::BadFormat);
    }
    if input.display_name.len() == 0 {
        invalid_account.display_name = Some(InvalidityKind::TooShort);
    }
    if invalid_account != InvalidNewAccount::default() {
        return Err(invalid_account);
    }
    Ok(())
}

fn check_account_update_input(input: &AccountUpdate) -> Result<(), InvalidAccountUpdate> {
    let mut invalid_account = InvalidAccountUpdate::default();
    if let Some(Password::ClearText(p)) = &input.new_password {
        if p.len() < 8 {
            invalid_account.new_password = Some(InvalidityKind::TooShort);
        }
    }
    if let Password::ClearText(p) = &input.old_password {
        if p.is_empty() {
            invalid_account.old_password = Some(InvalidityKind::Empty);
        }
    }
    if !EmailAddress::is_valid(&input.email) {
        invalid_account.email = Some(InvalidityKind::BadFormat);
    }
    if input.display_name.len() == 0 {
        invalid_account.display_name = Some(InvalidityKind::TooShort);
    }
    if invalid_account != InvalidAccountUpdate::default() {
        return Err(invalid_account);
    }
    Ok(())
}

impl App {
    pub async fn get_all_recipe_publishers(&self) -> Result<Vec<PublicAccountData>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_recipe_publishers(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_account(
        &self,
        new_account: &mut NewAccount,
    ) -> Result<i32, AppErrorWith<InvalidNewAccount>> {
        if let Err(e) = check_new_account_input(new_account) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        new_account.password =
            utils::hash_password(&new_account.password).map_err(|e| AppError::from(e))?;
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_account(&mut db_conn, new_account).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_account_by_id(&self, id: i32) -> Result<Option<Account>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_account_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_account_from_credentials(
        &self,
        cred: &Credentials,
    ) -> Result<Option<Account>, AppError> {
        let maybe_account = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_account_by_email(&mut db_conn, &cred.email).await
            },
            self.max_retry,
        )
        .await?;

        let account = match maybe_account {
            Some(v) => v,
            None => return Ok(None),
        };

        match utils::is_password_correct(&cred.password, &account.password) {
            Ok(true) => Ok(Some(account)),
            Ok(false) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn replace_account(
        &self,
        id: i32,
        account: &mut AccountUpdate,
    ) -> Result<Option<()>, AppErrorWith<InvalidAccountUpdate>> {
        if let Err(e) = check_account_update_input(account) {
            return Err(AppErrorWith::InvalidInput(e));
        };

        let maybe_db_account = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_account_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await?;

        let db_account = match maybe_db_account {
            Some(v) => v,
            None => return Ok(None),
        };

        match utils::is_password_correct(&account.old_password, &db_account.password) {
            Ok(true) => {}
            Ok(false) => {
                return Err(AppErrorWith::InvalidInput(InvalidAccountUpdate {
                    old_password: Some(InvalidityKind::Mismatch),
                    ..Default::default()
                }));
            }
            Err(e) => return Err(AppError::from(e).into()),
        };

        if let Some(v) = &account.new_password {
            account.new_password = Some(utils::hash_password(&v).map_err(|e| AppError::from(e))?);
        }

        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_account(&mut db_conn, id, account).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_account(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_account(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_recipe_to_account_favs(
        &self,
        account_id: i32,
        recipe_id: i32,
    ) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_recipe_to_account_favs(&mut db_conn, account_id, recipe_id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn remove_recipe_from_account_favs(
        &self,
        account_id: i32,
        recipe_id: i32,
    ) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::remove_recipe_from_account_favs(&mut db_conn, account_id, recipe_id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
