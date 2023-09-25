use std::convert::TryFrom;

use chrono::{DateTime, Utc};
use sqlx::{query, query_as, PgConnection};

use crate::models::{
    InvalidAccountValidationTokenUseRequest, InvalidPasswordResetTokenUseRequest,
    InvalidTokenNewRequest, Token, TokenType,
};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidAccountValidationTokenUseRequest {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

impl<'a> TryFrom<&DBConstraint> for InvalidPasswordResetTokenUseRequest {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

impl<'a> TryFrom<&DBConstraint> for InvalidTokenNewRequest {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn insert_new_token(
    db_conn: &mut PgConnection,
    account_id: i32,
    token_id: &str,
    token_type: &TokenType,
    valid_until: &DateTime<Utc>,
) -> Result<(), StorageError> {
    match token_type {
        TokenType::AccountValidation => {
            insert_new_validation_token(db_conn, account_id, token_id, valid_until).await?;
        }
        TokenType::PasswordReset => {
            insert_password_reset_token(db_conn, account_id, token_id, valid_until).await?;
        }
    };
    Ok(())
}

async fn insert_new_validation_token(
    db_conn: &mut PgConnection,
    account_id: i32,
    token_id: &str,
    valid_until: &DateTime<Utc>,
) -> Result<(), StorageError> {
    query!(
        "
            INSERT INTO account_validation_tokens (account_id, token, valid_until)
            VALUES ($1, $2, $3)
            ON CONFLICT (account_id) DO UPDATE SET
                token = excluded.token,
                valid_until = excluded.valid_until
        ",
        account_id,
        token_id,
        valid_until
    )
    .execute(db_conn)
    .await?;
    Ok(())
}

async fn insert_password_reset_token(
    db_conn: &mut PgConnection,
    account_id: i32,
    token_id: &str,
    valid_until: &DateTime<Utc>,
) -> Result<(), StorageError> {
    query!(
        "
            INSERT INTO password_reset_tokens (account_id, token, valid_until)
            VALUES ($1, $2, $3)
            ON CONFLICT (account_id) DO UPDATE SET
                token = excluded.token,
                valid_until = excluded.valid_until
        ",
        account_id,
        token_id,
        valid_until
    )
    .execute(db_conn)
    .await?;
    Ok(())
}

pub async fn get_token(
    db_conn: &mut PgConnection,
    token: &str,
    token_type: &TokenType,
) -> Result<Option<Token>, StorageError> {
    match token_type {
        TokenType::AccountValidation => get_account_validation_token(db_conn, token).await,
        TokenType::PasswordReset => get_password_reset_token(db_conn, token).await,
    }
}

pub async fn get_account_validation_token(
    db_conn: &mut PgConnection,
    token: &str,
) -> Result<Option<Token>, StorageError> {
    let row: Option<Token> = query_as!(
        Token,
        "
            SELECT
                token,
                account_id,
                valid_until
            FROM account_validation_tokens
            WHERE token = $1
        ",
        token
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn get_password_reset_token(
    db_conn: &mut PgConnection,
    token: &str,
) -> Result<Option<Token>, StorageError> {
    let row: Option<Token> = query_as!(
        Token,
        "
            SELECT
                token,
                account_id,
                valid_until
            FROM password_reset_tokens
            WHERE token = $1
        ",
        token
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn delete_token(
    db_conn: &mut PgConnection,
    token: &str,
    token_type: &TokenType,
) -> Result<(), StorageError> {
    match token_type {
        TokenType::AccountValidation => delete_account_validation_token(db_conn, token).await,
        TokenType::PasswordReset => delete_password_reset_token(db_conn, token).await,
    }
}

async fn delete_account_validation_token(
    db_conn: &mut PgConnection,
    token: &str,
) -> Result<(), StorageError> {
    query!(
        "
            DELETE FROM account_validation_tokens
            WHERE token = $1
        ",
        token
    )
    .execute(db_conn)
    .await?;

    Ok(())
}

async fn delete_password_reset_token(
    db_conn: &mut PgConnection,
    token: &str,
) -> Result<(), StorageError> {
    query!(
        "
            DELETE FROM password_reset_tokens
            WHERE token = $1
        ",
        token
    )
    .execute(db_conn)
    .await?;

    Ok(())
}
