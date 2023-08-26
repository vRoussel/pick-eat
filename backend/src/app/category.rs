use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

fn sanitize_category_input(input: &mut NewCategory) {
    input.name = sentence_case(&input.name);
}
fn check_category_input(input: &NewCategory) -> Result<(), InvalidCategory> {
    if input.name.is_empty() {
        return Err(InvalidCategory {
            name: Some(InvalidityKind::Empty),
        });
    };
    Ok(())
}

impl App {
    pub async fn get_all_categories(&self) -> Result<Vec<Category>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_categories(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_category(
        &self,
        new_category: &mut NewCategory,
    ) -> Result<i32, AppErrorWith<InvalidCategory>> {
        if let Err(e) = check_category_input(new_category) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_category_input(new_category);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_category(&mut db_conn, new_category).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_category(&self, id: i32) -> Result<Option<Category>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_category_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_category(
        &self,
        id: i32,
        new_category: &mut NewCategory,
    ) -> Result<Option<()>, AppErrorWith<InvalidCategory>> {
        if let Err(e) = check_category_input(new_category) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_category_input(new_category);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_category(&mut db_conn, id, new_category).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_category(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_category(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
