use super::{App, AppError};
use crate::{models::*, storage, utils::retry_up_to_n_times};

impl App {
    pub async fn get_all_seasons(&self) -> Result<Vec<Season>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_seasons(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_season(&self, id: i32) -> Result<Option<Season>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_season_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
