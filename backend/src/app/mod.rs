use sqlx::PgPool;
use thiserror::Error;

use crate::models::InvalidInput;
use crate::{email::EmailSender, storage::StorageError};

mod category;
mod diet;
mod season;
mod tag;
mod unit;

pub struct App {
    db_pool: PgPool,
    email_sender: EmailSender,
    max_retry: usize,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Internal Error ({0})")]
    InternalError(#[from] StorageError),
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
            StorageError::DBError { constraint, .. } => {
                match constraint.as_ref().and_then(|v| T::try_from(v).ok()) {
                    Some(t) => {
                        return Self::InvalidInput(t);
                    }
                    None => {}
                };
            }
            _ => {}
        };
        Self::AppError(value.into())
    }
}

impl App {
    pub fn new(db_pool: PgPool, email_sender: EmailSender) -> Self {
        Self {
            db_pool,
            email_sender,
            max_retry: 3,
        }
    }
}
