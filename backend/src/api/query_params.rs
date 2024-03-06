use std::{convert::TryFrom, fmt::Display, str::FromStr};

use serde::Deserialize;
use thiserror::Error;

use crate::models::{Range, RecipeFilters, SortMethod};

#[derive(Debug, Deserialize, Clone)]
pub struct SortMethodQueryParams {
    sort: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RangeQueryParams {
    range: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RecipeFiltersQueryParams {
    search: Option<String>,
    categories: Option<String>,
    seasons: Option<String>,
    ingredients: Option<String>,
    tags: Option<String>,
    account: Option<i32>,
    diets: Option<String>,
    only_favs: Option<bool>,
    only_private: Option<bool>,
    ids: Option<String>,
}

#[derive(Error, Debug)]
pub enum QueryParamError {
    #[error("Unable to parse {param} query param ({error})")]
    WrongFormat { param: &'static str, error: String },
    #[error("Bad value for {param} query param ({error})")]
    WrongValue { param: &'static str, error: String },
    #[error("Missing {param} query param ({error})")]
    Missing { param: &'static str, error: String },
}

impl TryFrom<RangeQueryParams> for Range {
    type Error = QueryParamError;

    fn try_from(value: RangeQueryParams) -> Result<Self, Self::Error> {
        let pair: Vec<i64> = value
            .range
            .split('-')
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()
            .map_err(|e| QueryParamError::WrongFormat {
                param: "range",
                error: e.to_string(),
            })?;

        if pair.len() != 2 {
            return Err(QueryParamError::WrongFormat {
                param: "range",
                error: "Wrong number of elements".to_owned(),
            });
        }

        Range::new(pair[0], pair[1]).map_err(|e| QueryParamError::WrongFormat {
            param: "range",
            error: e.to_string(),
        })
    }
}

fn parse_list<T, E>(input: &str, sep: &str) -> Result<Vec<T>, E>
where
    T: FromStr<Err = E>,
    E: Display,
{
    input
        .split(sep)
        .map(|s| s.parse::<T>())
        .collect::<Result<_, _>>()
}

impl TryFrom<RecipeFiltersQueryParams> for RecipeFilters {
    type Error = QueryParamError;

    fn try_from(value: RecipeFiltersQueryParams) -> Result<Self, Self::Error> {
        let mut filters: RecipeFilters = Default::default();
        if let Some(v) = value.categories {
            filters.categories = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "categories",
                    error: e.to_string(),
                })?;
        }
        if let Some(v) = value.seasons {
            filters.seasons = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "seasons",
                    error: e.to_string(),
                })?;
        }
        if let Some(v) = value.ingredients {
            filters.ingredients = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "ingredients",
                    error: e.to_string(),
                })?;
        }
        if let Some(v) = value.tags {
            filters.tags = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "tags",
                    error: e.to_string(),
                })?;
        }
        if let Some(v) = value.diets {
            filters.diets = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "diets",
                    error: e.to_string(),
                })?;
        }
        if let Some(v) = value.ids {
            filters.ids = parse_list::<i32, _>(&v, ",")
                .map(|x| Some(x))
                .map_err(|e| QueryParamError::WrongFormat {
                    param: "ids",
                    error: e.to_string(),
                })?;
        }
        filters.search = value.search;
        filters.account = value.account;
        filters.only_favs = value.only_favs.unwrap_or(false);
        filters.only_private = value.only_private.unwrap_or(false);
        Ok(filters)
    }
}

impl TryFrom<SortMethodQueryParams> for SortMethod {
    type Error = QueryParamError;

    fn try_from(value: SortMethodQueryParams) -> Result<Self, Self::Error> {
        match value.sort.as_ref() {
            "random" => Ok(SortMethod::Random),
            "name" => Ok(SortMethod::Name),
            "pub_date_asc" => Ok(SortMethod::PubDateAsc),
            "pub_date_desc" => Ok(SortMethod::PubDateDesc),
            "ingr_count" => Ok(SortMethod::IngrCount),
            "total_time" => Ok(SortMethod::TotalTime),
            _ => Err(QueryParamError::WrongValue {
                param: "sort",
                error: "unhandled sort query param value".to_owned(),
            }),
        }
    }
}
