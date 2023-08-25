mod tag;
//pub mod account;
//pub mod category;
//pub mod diet;
//pub mod ingredient;
//pub mod qingredient;
//pub mod recipe;
//pub mod season;
//pub mod token;
//pub mod unit;

use std::collections::HashMap;

use sqlx::postgres::PgDatabaseError;
pub use tag::*;

use sqlx::{self, Connection, Postgres, Transaction};
use sqlx::{postgres::PgConnection, query, Row};
use thiserror::Error;

use crate::models::{InvalidInput, InvalidityKind};
use crate::utils::RetryBehavior;

#[derive(Debug)]
pub struct DBConstraint(String);

impl std::fmt::Display for DBConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("database can't be reached ({0})")]
    Unreachable(String),
    #[error("[{code}] {message} ({detail})")]
    DBError {
        message: String,
        detail: String,
        code: String,
        constraint: Option<DBConstraint>,
    },
    #[error("{0}")]
    Other(String),
}

impl RetryBehavior for StorageError {
    fn is_retryable(&self) -> bool {
        match self {
            StorageError::Unreachable(_) => true,
            _ => false,
        }
    }
}

impl From<sqlx::Error> for StorageError {
    fn from(err: sqlx::Error) -> Self {
        use sqlx::Error::*;
        match err {
            Io(_) | Tls(_) | PoolTimedOut | PoolClosed => Self::Unreachable(err.to_string()),
            Database(db_error) => {
                let pg_error: &PgDatabaseError = db_error.downcast_ref();
                Self::DBError {
                    message: pg_error.message().to_owned(),
                    detail: pg_error.detail().unwrap_or_default().to_owned(),
                    code: pg_error.code().to_owned(),
                    constraint: pg_error.constraint().map(|v| DBConstraint(v.to_owned())),
                }
            }
            _ => Self::Other(err.to_string()),
        }
    }
}

pub async fn get_total_count(
    db_conn: &mut PgConnection,
    table_name: &str,
) -> Result<i64, Box<dyn std::error::Error>> {
    let q = format!("SELECT count(*) from {}", table_name);
    let count: i64 = query(&q).fetch_one(db_conn).await?.get(0);

    Ok(count)
}

pub enum IsolationLevel {
    RepeatableRead,
    Default,
}

pub async fn begin_transaction(
    db_conn: &mut PgConnection,
    isolation_lvl: IsolationLevel,
) -> Result<Transaction<Postgres>, StorageError> {
    let mut trans = db_conn.begin().await?;
    match isolation_lvl {
        IsolationLevel::RepeatableRead => {
            query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ;")
                .execute(&mut *trans)
                .await?;
        }
        IsolationLevel::Default => {}
    };
    Ok(trans)
}

pub async fn commit_transaction(trans: Transaction<'_, Postgres>) -> Result<(), StorageError> {
    trans.commit().await.map_err(|e| e.into())
}

pub async fn rollback_transaction(trans: Transaction<'_, Postgres>) -> Result<(), StorageError> {
    trans.rollback().await.map_err(|e| e.into())
}
