use std::error::Error;

use argon2::{
    password_hash::{self, rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use thiserror::Error;

use crate::models::Password;

pub trait RetryBehavior {
    fn is_retryable(&self) -> bool;
}

pub async fn retry_up_to_n_times<F, T, O, E>(mut func: F, max_tries: usize) -> T::Output
where
    F: FnMut() -> T,
    T: std::future::Future<Output = Result<O, E>>,
    E: Error + RetryBehavior,
{
    let mut fail_count = 0;
    loop {
        match func().await {
            Ok(t) => return Ok(t),
            Err(e) => {
                fail_count += 1;
                if fail_count >= max_tries {
                    return Err(e);
                }
            }
        }
    }
}

pub fn sentence_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str().to_lowercase().as_str(),
    }
}

#[derive(Debug, Error)]
pub enum PasswordHashingError {
    #[error("Password already hashed")]
    AlreadyHashed,
    #[error("BadHash ({0})")]
    BadHash(String),
    #[error("{0}")]
    Other(String),
}

pub fn hash_password(pwd: &Password) -> Result<Password, PasswordHashingError> {
    let pwd_clear = match pwd {
        Password::ClearText(v) => v,
        Password::Hashed(_) => {
            return Err(PasswordHashingError::AlreadyHashed);
        }
    };
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(pwd_clear.as_bytes(), &salt)
        .map_err(|e| PasswordHashingError::Other(e.to_string()))?
        .to_string();
    Ok(Password::Hashed(password_hash))
}

pub fn is_password_correct(pwd: &Password, hash_str: &str) -> Result<bool, PasswordHashingError> {
    let pwd_clear = match &pwd {
        Password::Hashed(_) => {
            return Err(PasswordHashingError::Other(
                "Found password hash where clear text was expected".to_owned(),
            )
            .into());
        }
        Password::ClearText(v) => v,
    };

    let hash = match PasswordHash::new(hash_str) {
        Ok(v) => v,
        Err(e) => {
            return Err(PasswordHashingError::BadHash(e.to_string()));
        }
    };
    if let Err(e) = Argon2::default().verify_password(pwd_clear.as_bytes(), &hash) {
        return Ok(false);
    }
    Ok(true)
    //if let Err(e) = Argon2::default().verify_password(old_password.as_bytes(), &hash) {
    //    return Err(AppErrorWith::InvalidInput(InvalidAccountUpdate {
    //        old_password: Some(InvalidityKind::Mismatch),
    //        ..Default::default()
    //    }));
    //}
}
