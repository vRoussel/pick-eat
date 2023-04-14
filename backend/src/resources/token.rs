use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::{query, query_as, Error, PgConnection};

#[derive(Debug)]
pub struct FromDB {
    pub account_id: i32,
    pub token: String,
    pub valid_until: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub email: String,
}

pub enum TokenValidity {
    Valid { account_id: i32 },
    Expired,
    Invalid,
}

pub async fn add_validation_token(
    db_conn: &mut PgConnection,
    req: &New,
) -> Result<Option<FromDB>, Error> {
    let token: Option<FromDB> = query_as!(
        FromDB,
        "
            INSERT INTO account_validation_tokens (account_id, token, valid_until)
            SELECT id, gen_random_uuid(), now() + interval '1 day'
            FROM (
                VALUES ($1)
            ) x (email)
            JOIN accounts USING (email)
            RETURNING account_id, token, valid_until
        ",
        req.email,
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(token)
}

pub async fn check_validation_token_validity(
    db_conn: &mut PgConnection,
    token: &str,
) -> Result<TokenValidity, Error> {
    let token = query_as!(
        FromDB,
        "
            SELECT account_id, token, valid_until
            FROM account_validation_tokens
            WHERE token = $1
        ",
        token
    )
    .fetch_optional(db_conn)
    .await?;

    let validity = match token {
        Some(t) => {
            if t.valid_until >= Utc::now() {
                TokenValidity::Valid {
                    account_id: t.account_id,
                }
            } else {
                TokenValidity::Expired
            }
        }
        None => TokenValidity::Invalid,
    };

    Ok(validity)
}

pub async fn delete_all_account_validation_tokens(
    db_conn: &mut PgConnection,
    account_id: i32,
) -> Result<(), Error> {
    query!(
        "
            DELETE FROM account_validation_tokens
            WHERE account_id = $1
        ",
        account_id
    )
    .execute(db_conn)
    .await?;

    Ok(())
}
