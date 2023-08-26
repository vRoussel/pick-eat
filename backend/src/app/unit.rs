use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

fn sanitize_unit_input(input: &mut NewUnit) {
    input.full_name = sentence_case(&input.full_name);
    if input.short_name.is_none() || input.short_name.as_ref().unwrap().is_empty() {
        input.short_name = Some(input.full_name.clone());
    }
}
fn check_unit_input(input: &NewUnit) -> Result<(), InvalidUnit> {
    let mut invalid_unit = InvalidUnit::default();
    if input.full_name.is_empty() {
        invalid_unit.full_name = Some(InvalidityKind::Empty);
    };
    if invalid_unit != InvalidUnit::default() {
        return Err(invalid_unit);
    }
    Ok(())
}

impl App {
    pub async fn get_all_units(&self) -> Result<Vec<Unit>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_units(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_unit(&self, new_unit: &mut NewUnit) -> Result<i32, AppErrorWith<InvalidUnit>> {
        if let Err(e) = check_unit_input(new_unit) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_unit_input(new_unit);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_unit(&mut db_conn, new_unit).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_unit(&self, id: i32) -> Result<Option<Unit>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_unit_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_unit(
        &self,
        id: i32,
        new_unit: &mut NewUnit,
    ) -> Result<Option<()>, AppErrorWith<InvalidUnit>> {
        if let Err(e) = check_unit_input(new_unit) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_unit_input(new_unit);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_unit(&mut db_conn, id, new_unit).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_unit(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_unit(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
