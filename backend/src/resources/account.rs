use std::fmt::Display;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgConnection;
use sqlx::{query, query_as, Error};

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    display_name: String,
    email: String,
    creation_date: time::Date,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    email: String,
    password: String,
    display_name: String,
}

pub type Ref = i32;

pub enum InsertAccountError {
    HashError(argon2::password_hash::Error),
    DBError(sqlx::Error),
}

impl Display for InsertAccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertAccountError::HashError(e) => write!(f, "{}", e),
            InsertAccountError::DBError(e) => write!(f, "{}", e),
        }
    }
}

impl From<argon2::password_hash::Error> for InsertAccountError {
    fn from(e: argon2::password_hash::Error) -> Self {
        InsertAccountError::HashError(e)
    }
}

impl From<sqlx::Error> for InsertAccountError {
    fn from(e: sqlx::Error) -> Self {
        InsertAccountError::DBError(e)
    }
}

pub async fn add_one(
    db_conn: &mut PgConnection,
    new_account: &New,
) -> Result<i32, InsertAccountError> {
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(new_account.password.as_bytes(), &salt)?
        .to_string();

    let new_id: i32 = query!(
        "
            INSERT INTO accounts (display_name, email, password)
                VALUES ($1, $2, $3)
            RETURNING id;
        ",
        new_account.display_name,
        new_account.email,
        password_hash
    )
    .fetch_one(db_conn)
    .await?
    .id;

    Ok(new_id)
}

pub async fn delete_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<()>, Error> {
    query!(
        "
            DELETE FROM accounts
            WHERE id = $1
        ",
        id
    )
    .execute(db_conn)
    .await?;
    Ok(Some(()))
}

pub enum CheckCredentialsError {
    HashError(argon2::password_hash::Error),
    DBError(sqlx::Error),
}

impl Display for CheckCredentialsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckCredentialsError::HashError(e) => write!(f, "{}", e),
            CheckCredentialsError::DBError(e) => write!(f, "{}", e),
        }
    }
}

impl From<argon2::password_hash::Error> for CheckCredentialsError {
    fn from(e: argon2::password_hash::Error) -> Self {
        CheckCredentialsError::HashError(e)
    }
}

impl From<sqlx::Error> for CheckCredentialsError {
    fn from(e: sqlx::Error) -> Self {
        CheckCredentialsError::DBError(e)
    }
}

pub async fn check_credentials(
    db_conn: &mut PgConnection,
    email: &str,
    password: &str,
) -> Result<i32, CheckCredentialsError> {
    let row = query!(
        "
            SELECT id, password FROM accounts WHERE email = $1
        ",
        email
    )
    .fetch_one(db_conn)
    .await?;

    let hash = PasswordHash::new(&row.password)?;
    Argon2::default().verify_password(password.as_bytes(), &hash)?;

    Ok(row.id)
}

pub async fn get_one(db_conn: &mut PgConnection, id: i32) -> Result<Option<FromDB>, Error> {
    let row: Option<FromDB> = query_as!(
        FromDB,
        "
            SELECT
                email,
                display_name,
                creation_date
            FROM accounts
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}
