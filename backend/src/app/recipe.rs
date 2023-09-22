use std::sync::atomic::Ordering;

use super::{App, AppError, AppErrorWith};
use crate::{
    models::*,
    storage,
    utils::{retry_up_to_n_times, sentence_case},
};

static MAX_RECIPES_FETCH_SIZE: i64 = 50;

fn sanitize_recipe_input(input: &mut NewRecipe) {
    input.name = sentence_case(&input.name);
}
fn check_recipe_input(input: &NewRecipe) -> Result<(), InvalidRecipe> {
    let mut invalid_recipe = InvalidRecipe::default();
    if input.name.is_empty() {
        invalid_recipe.name = Some(InvalidityKind::Empty);
    };
    if input.preparation_time_min <= 0 {
        invalid_recipe.preparation_time_min = Some(InvalidityKind::BadValue);
    }
    if input.cooking_time_min < 0 {
        invalid_recipe.cooking_time_min = Some(InvalidityKind::BadValue);
    }
    if input.n_shares <= 0 {
        invalid_recipe.n_shares = Some(InvalidityKind::BadValue);
    }
    if input.categories.is_empty() {
        invalid_recipe.categories = Some(InvalidityKind::Empty);
    }
    if input.seasons.is_empty() {
        invalid_recipe.seasons = Some(InvalidityKind::Empty);
    }
    if input.ingredients.is_empty() {
        invalid_recipe.ingredients = Some(InvalidityKind::Empty);
    }
    if input.instructions.is_empty() {
        invalid_recipe.instructions = Some(InvalidityKind::Empty);
    }
    if invalid_recipe != InvalidRecipe::default() {
        return Err(invalid_recipe);
    }
    Ok(())
}

impl App {
    pub fn max_recipe_fetch_size(&self) -> i64 {
        return MAX_RECIPES_FETCH_SIZE;
    }
    pub fn validate_recipe_range(&self, range: &Range) -> Result<(), RangeError> {
        range.validate(Some(MAX_RECIPES_FETCH_SIZE), Some(self.get_recipe_count()))
    }
    pub async fn get_many_recipes(
        &self,
        range: &Range,
        filters: &mut RecipeFilters,
        sort_method: SortMethod,
        account_id: Option<i32>,
    ) -> Result<Vec<RecipeSummary>, AppError> {
        if let Some(query) = &filters.search {
            filters.search = Some(query.split_whitespace().collect::<Vec<_>>().join(" "));
        };

        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_many_recipes(
                    &mut db_conn,
                    range,
                    filters,
                    sort_method.to_owned(),
                    account_id,
                )
                .await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn add_recipe(
        &self,
        new_recipe: &mut NewRecipe,
        account_id: i32,
    ) -> Result<i32, AppErrorWith<InvalidRecipe>> {
        if let Err(e) = check_recipe_input(new_recipe) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_recipe_input(new_recipe);
        let new_id = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::add_recipe(&mut db_conn, new_recipe, account_id).await
            },
            self.max_retry,
        )
        .await?;

        self.recipe_count.fetch_add(1, Ordering::Relaxed);
        Ok(new_id)
    }

    pub async fn get_recipe(
        &self,
        id: i32,
        account_id: Option<i32>,
    ) -> Result<Option<Recipe>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_recipe_by_id(&mut db_conn, id, account_id).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn replace_recipe(
        &self,
        recipe_id: i32,
        new_recipe: &mut NewRecipe,
        account_id: i32,
        is_admin: bool,
    ) -> Result<Option<()>, AppErrorWith<InvalidRecipe>> {
        let author_id = retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::get_recipe_author(&mut db_conn, recipe_id).await
            },
            self.max_retry,
        )
        .await?;

        if !(is_admin || author_id == account_id) {
            return Err(AppError::NotAllowed.into());
        }

        if let Err(e) = check_recipe_input(new_recipe) {
            return Err(AppErrorWith::InvalidInput(e));
        };
        sanitize_recipe_input(new_recipe);

        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::replace_recipe(&mut db_conn, recipe_id, new_recipe).await
            },
            self.max_retry,
        )
        .await
        .map_err(|e| e.into())
    }

    pub async fn delete_recipe(&self, id: i32) -> Result<Option<()>, AppError> {
        retry_up_to_n_times(
            || async {
                let mut db_conn = self.db_pool.acquire().await?;
                storage::delete_recipe(&mut db_conn, id).await
            },
            self.max_retry,
        )
        .await?;

        self.recipe_count.fetch_sub(1, Ordering::Relaxed);
        Ok(Some(()))
    }
}
