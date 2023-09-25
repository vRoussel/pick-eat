use sqlx::postgres::PgConnection;
use sqlx::query_as;

use crate::models::Season;

use super::StorageError;

pub async fn get_all_seasons(db_conn: &mut PgConnection) -> Result<Vec<Season>, StorageError> {
    let rows: Vec<Season> = query_as!(
        Season,
        "
            SELECT
                id,
                name,
                label
            FROM seasons
            ORDER BY id
        ",
    )
    .fetch_all(db_conn)
    .await?;

    Ok(rows)
}

pub async fn get_season_by_id(
    db_conn: &mut PgConnection,
    id: i32,
) -> Result<Option<Season>, StorageError> {
    let row: Option<Season> = query_as!(
        Season,
        "
            SELECT
                id,
                name,
                label
            FROM seasons
            WHERE id = $1
        ",
        id
    )
    .fetch_optional(db_conn)
    .await?;

    Ok(row)
}
