use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

fn sanitize_tag_input(input: &mut NewTag) {
    input.name = sentence_case(&input.name);
}
impl App {
    pub async fn get_all_tags(&self) -> Result<Vec<Tag>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_tags(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_tag(&self, new_tag: &mut NewTag) -> Result<i32, AppErrorWith<InvalidTag>> {
        sanitize_tag_input(new_tag);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_tag(&mut db_conn, new_tag).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_tag(&self, id: i32) -> Result<Option<Tag>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_tag_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_tag(
        &self,
        id: i32,
        new_tag: &mut NewTag,
    ) -> Result<Option<()>, AppErrorWith<InvalidTag>> {
        sanitize_tag_input(new_tag);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_tag(&mut db_conn, id, new_tag).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_tag(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_tag(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
