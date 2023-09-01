use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{self, retry_up_to_n_times},
};
use chrono::{prelude::*, Duration};
use email_address::EmailAddress;
use uuid::Uuid;

impl App {
    pub async fn generate_token(
        &self,
        token_req: &TokenNewRequest,
        token_type: TokenType,
        base_url: &str,
    ) -> Result<(), AppErrorWith<InvalidTokenNewRequest>> {
        if !EmailAddress::is_valid(&token_req.email) {
            let invalid_req = InvalidTokenNewRequest {
                email: Some(InvalidityKind::BadFormat),
            };
            return Err(AppErrorWith::InvalidInput(invalid_req));
        }

        let uuid = Uuid::new_v4().to_string();
        let valid_until = Utc::now() + Duration::days(1);
        let maybe_account = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_account_by_email(&mut db_conn, &token_req.email).await
            },
            self.max_retry,
        )
        .await?;

        if let Some(account) = maybe_account {
            retry_up_to_n_times(
                || async {
                    let mut db_conn = self.db_pool.acquire().await?;
                    storage::insert_new_token(
                        &mut db_conn,
                        account.id,
                        &uuid,
                        &token_type,
                        &valid_until,
                    )
                    .await
                },
                self.max_retry,
            )
            .await?;
            match token_type {
                TokenType::AccountValidation => {
                    self.email_sender
                        .send_account_validation_email(&token_req.email, &uuid, &base_url)
                        .await
                        .map_err(|e| AppErrorWith::from(AppError::Other(e)))?;
                }
                TokenType::PasswordReset => {
                    self.email_sender
                        .send_password_reset_email(&token_req.email, &uuid, &base_url)
                        .await
                        .map_err(|e| AppErrorWith::from(AppError::Other(e)))?;
                }
            };
        }
        Ok(())
    }

    pub async fn use_account_validation_token(
        &self,
        token_req: &AccountValidationTokenUseRequest,
    ) -> Result<(), AppErrorWith<InvalidAccountValidationTokenUseRequest>> {
        let maybe_token = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_token(
                    &mut db_conn,
                    &token_req.token,
                    &TokenType::AccountValidation,
                )
                .await
            },
            self.max_retry,
        )
        .await?;

        let token = match maybe_token {
            Some(t) => t,
            None => {
                let invalid_req = InvalidAccountValidationTokenUseRequest {
                    token: Some(InvalidityKind::NotFound),
                };
                return Err(AppErrorWith::InvalidInput(invalid_req));
            }
        };

        if token.valid_until < Utc::now() {
            let invalid_req = InvalidAccountValidationTokenUseRequest {
                token: Some(InvalidityKind::Expired),
            };
            return Err(AppErrorWith::InvalidInput(invalid_req));
        }

        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                let mut transaction =
                    storage::begin_transaction(&mut db_conn, storage::IsolationLevel::Default)
                        .await?;
                storage::validate_account(&mut *transaction, token.account_id).await?;
                storage::delete_token(
                    &mut *transaction,
                    &token_req.token,
                    &TokenType::AccountValidation,
                )
                .await?;
                storage::commit_transaction(transaction).await
            },
            self.max_retry,
        )
        .await?;
        Ok(())
    }

    pub async fn use_password_reset_token(
        &self,
        token_req: &mut PasswordResetTokenUseRequest,
    ) -> Result<(), AppErrorWith<InvalidPasswordResetTokenUseRequest>> {
        if let Password::ClearText(p) = &token_req.new_password {
            if p.len() < 8 {
                let invalid_req = InvalidPasswordResetTokenUseRequest {
                    new_password: Some(InvalidityKind::NotFound),
                    ..Default::default()
                };
                return Err(AppErrorWith::InvalidInput(invalid_req));
            }
        }

        let maybe_token = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_token(&mut db_conn, &token_req.token, &TokenType::PasswordReset).await
            },
            self.max_retry,
        )
        .await?;

        let token = match maybe_token {
            Some(t) => t,
            None => {
                let invalid_req = InvalidPasswordResetTokenUseRequest {
                    token: Some(InvalidityKind::NotFound),
                    ..Default::default()
                };
                return Err(AppErrorWith::InvalidInput(invalid_req));
            }
        };

        if token.valid_until < Utc::now() {
            let invalid_req = InvalidPasswordResetTokenUseRequest {
                token: Some(InvalidityKind::Expired),
                ..Default::default()
            };
            return Err(AppErrorWith::InvalidInput(invalid_req));
        }

        token_req.new_password =
            utils::hash_password(&token_req.new_password).map_err(|e| AppError::from(e))?;

        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                let mut transaction =
                    storage::begin_transaction(&mut db_conn, storage::IsolationLevel::Default)
                        .await?;
                storage::replace_account_password(
                    &mut *transaction,
                    token.account_id,
                    &token_req.new_password,
                )
                .await?;
                storage::delete_token(
                    &mut *transaction,
                    &token_req.token,
                    &TokenType::PasswordReset,
                )
                .await?;
                storage::commit_transaction(transaction).await
            },
            self.max_retry,
        )
        .await?;
        Ok(())
    }
}
