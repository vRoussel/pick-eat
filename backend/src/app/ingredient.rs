use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

fn sanitize_ingredient_input(input: &mut NewIngredient) {
    input.name = sentence_case(&input.name);
}
fn check_ingredient_input(input: &NewIngredient) -> Result<(), InvalidIngredient> {
    if input.name.is_empty() {
        return Err(InvalidIngredient {
            name: Some(InvalidityKind::Empty),
            ..Default::default()
        });
    };
    Ok(())
}

impl App {
    pub async fn get_all_ingredients(&self) -> Result<Vec<Ingredient>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_all_ingredients(&mut db_conn).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_ingredient(
        &self,
        new_ingredient: &mut NewIngredient,
    ) -> Result<i32, AppErrorWith<InvalidIngredient>> {
        if let Err(e) = check_ingredient_input(new_ingredient) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_ingredient_input(new_ingredient);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_ingredient(&mut db_conn, new_ingredient).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn get_ingredient(&self, id: i32) -> Result<Option<Ingredient>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_ingredient_by_id(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_ingredient(
        &self,
        id: i32,
        new_ingredient: &mut NewIngredient,
    ) -> Result<Option<()>, AppErrorWith<InvalidIngredient>> {
        if let Err(e) = check_ingredient_input(new_ingredient) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_ingredient_input(new_ingredient);
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_ingredient(&mut db_conn, id, new_ingredient).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_ingredient(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_ingredient(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }
}
