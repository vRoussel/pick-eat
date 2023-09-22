use std::sync::atomic::{AtomicI64, Ordering};

use log::warn;
use sqlx::PgPool;
use thiserror::Error;

use crate::models::InvalidInput;
use crate::storage;
use crate::utils::{retry_up_to_n_times, PasswordHashingError};
use crate::{email::EmailSender, storage::StorageError};

mod account;
mod category;
mod diet;
mod ingredient;
mod season;
mod tag;
mod token;
mod unit;

pub struct App {
    db_pool: PgPool,
    email_sender: EmailSender,
    max_retry: usize,
    recipe_count: AtomicI64,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    PasswordHashingError(#[from] PasswordHashingError),
    #[error("Unknown App Error ({0})")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum AppErrorWith<T: InvalidInput> {
    #[error("Invalid Input ({0})")]
    InvalidInput(T),
    #[error(transparent)]
    AppError(#[from] AppError),
}

impl<T: InvalidInput> From<StorageError> for AppErrorWith<T> {
    fn from(value: StorageError) -> Self {
        match &value {
            StorageError::DBError { constraint, .. } if constraint.is_some() => {
                match T::try_from(constraint.as_ref().unwrap()) {
                    Ok(v) => {
                        return Self::InvalidInput(v);
                    }
                    Err(e) => {
                        warn!("{}", e);
                    }
                };
            }
            _ => {}
        };
        Self::AppError(value.into())
    }
}

impl App {
    pub async fn new(db_pool: PgPool, email_sender: EmailSender) -> Result<Self, AppError> {
        let total_count = retry_up_to_n_times(
            || async {
                let mut db_conn = db_pool.acquire().await?;
                storage::get_recipes_count(&mut db_conn).await
            },
            3,
        )
        .await?;
        Ok(Self {
            db_pool,
            email_sender,
            max_retry: 3,
            recipe_count: AtomicI64::new(total_count),
        })
    }

    pub fn get_recipe_count(&self) -> i64 {
        return self.recipe_count.load(Ordering::Relaxed);
    }
}
