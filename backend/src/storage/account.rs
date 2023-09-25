use std::convert::TryFrom;

use sqlx::{postgres::PgConnection, query, query_as, Connection};

use crate::models::{
    Account, AccountUpdate, InvalidAccountUpdate, InvalidNewAccount, InvalidityKind, NewAccount,
    Password, PublicAccountData,
};

use super::{DBConstraint, StorageError};

impl<'a> TryFrom<&DBConstraint> for InvalidNewAccount {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "accounts_uq_display_name" => Ok(InvalidNewAccount {
                display_name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            "accounts_uq_email" => Ok(InvalidNewAccount {
                email: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

impl<'a> TryFrom<&DBConstraint> for InvalidAccountUpdate {
    type Error = String;

    fn try_from(value: &DBConstraint) -> Result<Self, String> {
        match value.0.as_str() {
            "accounts_uq_display_name" => Ok(InvalidAccountUpdate {
                display_name: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            "accounts_uq_email" => Ok(InvalidAccountUpdate {
                email: Some(InvalidityKind::AlreadyUsed),
                ..Default::default()
            }),
            _ => Err(format!("Unknown DB constraint {}", value.0)),
        }
    }
}

pub async fn get_all_recipe_publishers(
    db_conn: &mut PgConnection,
) -> Result<Vec<PublicAccountData>, StorageError> {
    let rows: Vec<PublicAccountData> = query_as!(
        PublicAccountData,
        "
            SELECT
                distinct(author_id) as id,
                display_name
            FROM recipes r
            INNER JOIN accounts a on r.author_id = a.id
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn add_account(
    db_conn: &mut PgConnection,
    new_account: &NewAccount,
) -> Result<i32, StorageError> {
    let password_hash = match &new_account.password {
        Password::ClearText(_) => {
            return Err(StorageError::Other(
                "Expected hashed password and got clear text".to_owned(),
            ));
        }
        Password::Hashed(v) => v,
    };

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

pub async fn get_account_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Account>, StorageError> {
    let row: Option<Account> = query_as!(
        Account,
        "
            SELECT
                id,
                password,
                email,
                display_name,
                creation_date,
                is_admin,
                is_validated
            FROM accounts
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn get_account_by_email(
    db_conn: &mut PgConnection,
    email: &str,
) -> Result<Option<Account>, StorageError> {
    let row: Option<Account> = query_as!(
        Account,
        "
            SELECT
                id,
                password,
                email,
                display_name,
                creation_date,
                is_admin,
                is_validated
            FROM accounts
            WHERE email = $1
        ",
        email
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}

pub async fn replace_account(
    db_conn: &mut PgConnection,
    id: i32,
    account: &AccountUpdate,
) -> Result<Option<()>, StorageError> {
    let mut transaction = db_conn.begin().await?;

    let mut n_rows: u64 = query!(
        "
            UPDATE accounts SET
                display_name = $1,
                email = $2
            WHERE id = $3
        ",
        account.display_name,
        account.email,
        id
    )
    .execute(&mut *transaction)
    .await?
    .rows_affected();

    if let Some(new_password) = &account.new_password {
        let password_hash = match new_password {
            Password::ClearText(_) => {
                return Err(StorageError::Other(
                    "Expected hashed password and got clear text".to_owned(),
                ));
            }
            Password::Hashed(v) => v,
        };

        n_rows += query!(
            "
                UPDATE accounts SET
                password = $1
                WHERE id = $2
            ",
            password_hash,
            id
        )
        .execute(&mut *transaction)
        .await?
        .rows_affected();
    };

    transaction.commit().await?;

    if n_rows > 0 {
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

pub async fn replace_account_password(
    db_conn: &mut PgConnection,
    id: i32,
    new_password: &Password,
) -> Result<Option<()>, StorageError> {
    let password_hash = match new_password {
        Password::ClearText(_) => {
            return Err(StorageError::Other(
                "Expected hashed password and got clear text".to_owned(),
            ));
        }
        Password::Hashed(v) => v,
    };

    let n_rows = query!(
        "
            UPDATE accounts SET
                password = $1
            WHERE id = $2
        ",
        password_hash,
        id
    )
    .execute(db_conn)
    .await?
    .rows_affected();
    if n_rows > 0 {
        Ok(Some(()))
    } else {
        Ok(None)
    }
}

pub async fn validate_account(db_conn: &mut PgConnection, id: i32) -> Result<(), StorageError> {
    query!(
        "
            UPDATE accounts SET
                is_validated = 't'
            WHERE id = $1
        ",
        id
    )
    .execute(db_conn)
    .await?;
    Ok(())
}

pub async fn delete_account(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<()>, StorageError> {
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

pub async fn add_recipe_to_account_favs(
    db_conn: &mut PgConnection,
    account_id: i32,
    recipe_id: i32,
) -> Result<Option<()>, StorageError> {
    query!(
        "
            INSERT INTO accounts_fav_recipes (account_id, recipe_id)
            VALUES ($1, $2)
        ",
        account_id,
        recipe_id
    )
    .execute(db_conn)
    .await?;
    Ok(Some(()))
}

pub async fn remove_recipe_from_account_favs(
    db_conn: &mut PgConnection,
    account_id: i32,
    recipe_id: i32,
) -> Result<Option<()>, StorageError> {
    query!(
        "
            DELETE FROM accounts_fav_recipes
            WHERE
                account_id = $1 AND
                recipe_id = $2
        ",
        account_id,
        recipe_id
    )
    .execute(db_conn)
    .await?;
    Ok(Some(()))
}
