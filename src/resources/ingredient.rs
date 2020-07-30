use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, error::Error};
use super::unit;

#[derive(Debug, Serialize, Deserialize)]
pub struct FromDB {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) default_unit: Option<unit::FromDB>
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct New {
    pub(crate) name: String,
    pub(crate) default_unit_id: Option<i32>
}

pub mod quantified {
    use serde::{Deserialize, Serialize};
    use super::unit;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Full {
        pub(crate) id: i32,
        pub(crate) name: String,
        pub(crate) quantity: Option<i16>,
        pub(crate) unit: Option<unit::FromDB>
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Ref {
        pub(crate) id: i32,
        pub(crate) quantity: Option<i16>,
        pub(crate) unit_id: Option<i32>
    }
}


impl From<&tokio_postgres::row::Row> for FromDB {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let default_unit = match row.try_get("default_unit_id") {
            Ok(unit_id) => Some(
                unit::FromDB {
                    id: unit_id,
                    full_name: row.get("default_unit_full_name"),
                    short_name: row.get("default_unit_short_name")
                }
            ),
            Err(_) => None
        };

        FromDB {
            id: row.get("id"),
            name: row.get("name"),
            default_unit: default_unit
        }
    }
}

impl From<&tokio_postgres::row::Row> for quantified::Full {
    fn from(row: &tokio_postgres::row::Row) -> Self {
        let unit = match row.try_get("unit_id") {
            Ok(unit_id) => Some(
                unit::FromDB {
                    id: unit_id,
                    full_name: row.get("unit_full_name"),
                    short_name: row.get("unit_short_name")
                }
            ),
            Err(_) => None
        };

        quantified::Full {
            id: row.get("id"),
            name: row.get("name"),
            quantity: row.get("quantity"),
            unit: unit
        }
    }
}

pub async fn get_many(db_conn: &Client, min: i64, max: i64) -> Result<Vec<FromDB>, Error> {
    let ingredients_query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            u.id as default_unit_id, \
            u.full_name as default_unit_full_name, \
            u.short_name as default_unit_short_name \
        FROM
            ingredients as i \
            LEFT JOIN units as u \
            ON i.default_unit_id = u.id \
        ORDER BY name
        OFFSET $1
        LIMIT $2
    ";

    db_conn.query(ingredients_query, &[&(min-1), &(max-min+1)])
        .await
        .map(|rows|rows.iter().map(|r| r.into()).collect())
}

pub async fn add_one(db_conn: &Client, new_ingredient: &New) -> Result<i32, Error> {
    let insert_query = "\
        INSERT INTO ingredients (name, default_unit_id) \
            VALUES ($1, $2) \
        RETURNING id;
    ";
    db_conn.query(insert_query, &[&new_ingredient.name, &new_ingredient.default_unit_id])
        .await
        .map(|rows| rows[0].get(0))
}

pub async fn get_one(db_conn: &Client, id: i32) -> Result<Option<FromDB>, Error> {
    let query = "\
        SELECT \
            i.id as id, \
            i.name as name, \
            u.id as default_unit_id, \
            u.full_name as default_unit_full_name, \
            u.short_name as default_unit_short_name \
        FROM
            ingredients as i \
            LEFT JOIN units as u \
            ON i.default_unit_id = u.id \
        WHERE
            i.id = $1 \
    ";

    db_conn.query_opt(query, &[&id])
        .await
        .map(|opt| opt.map(|ref row| row.into()))
}

pub async fn modify_one(db_conn: &Client, id: i32, new_ingredient: &New) -> Result<Option<()>, Error> {
    let update_query = "\
        UPDATE ingredients SET \
            name = $1, \
            default_unit_id = $2, \
        WHERE id = $3 \
        RETURNING id;
    ";
    db_conn.query_opt(update_query, &[&new_ingredient.name, &new_ingredient.default_unit_id, &id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}

pub async fn delete_one(db_conn: &Client, id: i32) -> Result<Option<()>, Error> {
    let delete_query = "\
        DELETE FROM ingredients \
        WHERE id = $1 \
        RETURNING id;
    ";
    db_conn.query_opt(delete_query, &[&id])
        .await
        .map(|opt| opt.map(|_| ())) // OK(Some(row)) => Ok(Some(()))
}
