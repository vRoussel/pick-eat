use sqlx::{
    postgres::{PgHasArrayType, PgTypeInfo},
    prelude::FromRow,
};

use super::{Category, Diet, InvalidInput, InvalidityKind, Season, Tag, Unit};

#[derive(Debug)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub notes: String,
    pub preparation_time_min: i16,
    pub cooking_time_min: i16,
    pub image: String,
    pub publication_date: chrono::NaiveDate,
    pub update_date: Option<chrono::NaiveDate>,
    pub instructions: Vec<String>,
    pub n_shares: i16,
    pub shares_unit: String,
    pub is_favorite: bool,
    pub is_private: bool,
    pub ingredients: Vec<QIngredient>,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub seasons: Vec<Season>,
    pub author_id: i32,
    pub author_name: String,
    pub diets: Vec<Diet>,
}

#[derive(Debug)]
pub struct NewRecipe {
    pub name: String,
    pub notes: String,
    pub preparation_time_min: i16,
    pub cooking_time_min: i16,
    pub image: String,
    pub instructions: Vec<String>,
    pub n_shares: i16,
    pub shares_unit: String,
    pub is_private: bool,
    pub ingredients: Vec<NewQIngredient>,
    pub categories: Vec<i32>,
    pub tags: Vec<i32>,
    pub seasons: Vec<i32>,
    pub diets: Vec<i32>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidRecipe {
    pub name: Option<InvalidityKind>,
    pub notes: Option<InvalidityKind>,
    pub preparation_time_min: Option<InvalidityKind>,
    pub cooking_time_min: Option<InvalidityKind>,
    pub image: Option<InvalidityKind>,
    pub instructions: Option<InvalidityKind>,
    pub n_shares: Option<InvalidityKind>,
    pub shares_unit: Option<InvalidityKind>,
    pub is_private: Option<InvalidityKind>,
    pub ingredients: Option<InvalidityKind>,
    pub categories: Option<InvalidityKind>,
    pub tags: Option<InvalidityKind>,
    pub seasons: Option<InvalidityKind>,
    pub author_id: Option<InvalidityKind>,
    pub author_name: Option<InvalidityKind>,
    pub diets: Option<InvalidityKind>,
}

impl InvalidInput for InvalidRecipe {}

#[derive(Debug, FromRow)]
pub struct RecipeSummary {
    pub id: i32,
    pub name: String,
    pub image: String,
    pub n_shares: i16,
    pub shares_unit: String,
    pub is_favorite: bool,
    pub is_private: bool,
    pub diets: Vec<Diet>,
    pub ingredients: Vec<QIngredient>,
    pub total_count: i64,
}

#[derive(Debug)]
pub struct QIngredient {
    pub id: i32,
    pub name: String,
    pub quantity: Option<f32>,
    pub unit: Option<Unit>,
}

#[derive(Debug)]
pub struct NewQIngredient {
    pub id: i32,
    pub quantity: Option<f32>,
    pub unit_id: Option<i32>,
}

#[derive(Debug, Default)]
pub struct RecipeFilters {
    pub search: Option<String>,
    pub categories: Option<Vec<i32>>,
    pub seasons: Option<Vec<i32>>,
    pub ingredients: Option<Vec<i32>>,
    pub tags: Option<Vec<i32>>,
    pub account: Option<i32>,
    pub diets: Option<Vec<i32>>,
    pub only_favs: bool,
    pub ids: Option<Vec<i32>>,
    //TODO mes recettes privéees
}

#[derive(Debug, Clone)]
pub enum SortMethod {
    Random,
    Name,
    PubDateAsc,
    PubDateDesc,
    IngrCount,
    TotalTime,
}

// We can't use #[derive(Type)] here
// https://github.com/launchbadge/sqlx/issues/1031

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for QIngredient
where
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<f32>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<f32>: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<Unit>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<Unit>: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let quantity = decoder.try_decode::<Option<f32>>()?;
        let unit = decoder.try_decode::<Option<Unit>>()?;
        ::std::result::Result::Ok(QIngredient {
            id,
            name,
            quantity,
            unit,
        })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for QIngredient {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("qingredient")
    }
}

impl PgHasArrayType for QIngredient {
    fn array_type_info() -> PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("_qingredient")
    }
}

//#[derive(Debug)]
//pub struct NewRecipe {
//    pub name: String,
//}
//
//#[derive(Debug)]
//pub struct InvalidRecipe {
//    pub name: Option<InvalidityKind>,
//}
//
//impl InvalidInput for InvalidRecipe {}
