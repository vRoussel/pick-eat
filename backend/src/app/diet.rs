use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

fn sanitize_diet_input(input: &mut NewDiet) {
    input.name = sentence_case(&input.name);
}
fn check_diet_input(input: &NewDiet) -> Result<(), InvalidDiet> {
    let mut invalid_diet = InvalidDiet::default();
    if input.name.is_empty() {
        invalid_diet.name = Some(InvalidityKind::Empty);
    }

    if let Some(l) = &input.label {
        if l.is_empty() {
            invalid_diet.label = Some(InvalidityKind::Empty);
        }
    }

    if invalid_diet != InvalidDiet::default() {
        return Err(invalid_diet);
    }
    Ok(())
}

impl App {
    pub async fn get_all_diets(&self) -> Result<Vec<Diet>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_diets(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_diet(&self, new_diet: &mut NewDiet) -> Result<i32, AppErrorWith<InvalidDiet>> {
        if let Err(e) = check_diet_input(new_diet) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_diet_input(new_diet);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_diet(&mut db_conn, new_diet).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_diet(&self, id: i32) -> Result<Option<Diet>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_diet_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_diet(
        &self,
        id: i32,
        new_diet: &mut NewDiet,
    ) -> Result<Option<()>, AppErrorWith<InvalidDiet>> {
        if let Err(e) = check_diet_input(new_diet) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_diet_input(new_diet);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_diet(&mut db_conn, id, new_diet).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_diet(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_diet(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
