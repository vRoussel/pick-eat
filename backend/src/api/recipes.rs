use core::fmt;
use std::convert::TryInto;

use super::categories::CategoryOut;
use super::diets::DietOut;
use super::seasons::SeasonOut;
use super::tags::TagOut;
use super::units::UnitOut;
use super::APIError;
use actix_web::{delete, get, http, post, put, web, HttpResponse, Responder};
use log::*;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json;

use crate::api::query_params::{RangeQueryParams, RecipeFiltersQueryParams, SortMethodQueryParams};
use crate::api::{Admin, User};
use crate::app::{App, AppErrorWith};
use crate::models::{self, InvalidRecipe, Range, RangeError, RecipeFilters, SortMethod};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_recipe)
        .service(add_recipe)
        .service(get_many_recipes)
        .service(replace_recipe)
        .service(delete_recipe);
}

impl From<InvalidRecipe> for Vec<APIError> {
    fn from(value: InvalidRecipe) -> Self {
        type Kind = models::InvalidityKind;
        let mut ret = Vec::new();
        if let Some(v) = value.name {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez donner un nom à votre recette",
                        field: Some("name"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's name, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.preparation_time_min {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le temps de préparation doit être d'au moins 1 minute",
                        field: Some("prep_time_min"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's prep_time_min, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.cooking_time_min {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le temps de cuisson ne peut pas être négatif",
                        field: Some("cook_time_min"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's cook_time_min, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.n_shares {
            match v {
                Kind::BadValue => {
                    ret.push(APIError {
                        message: "Le nombre de part doit être au moins de 1",
                        field: Some("n_shares"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's n_shares, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.categories {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez selectionner au moins une catégorie",
                        field: Some("category_ids"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's categories, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.seasons {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez selectionner au moins une saison",
                        field: Some("season_ids"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's seasons, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.ingredients {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez ajouter au moins un ingrédient",
                        field: Some("q_ingredients"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's ingredients, this should not happen",
                        v
                    );
                }
            };
        };
        if let Some(v) = value.instructions {
            match v {
                Kind::Empty => {
                    ret.push(APIError {
                        message: "Veuillez renseigner les instructions pour réaliser la recette",
                        field: Some("instructions"),
                        code: None,
                    });
                }
                _ => {
                    warn!(
                        "{:?} error received for recipe's categories, this should not happen",
                        v
                    );
                }
            };
        };
        ret
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeOut {
    id: i32,
    name: String,
    notes: String,
    preparation_time_min: i16,
    cooking_time_min: i16,
    image: String,
    publication_date: chrono::NaiveDate,
    instructions: Vec<String>,
    n_shares: i16,
    is_favorite: bool,
    is_private: bool,
    ingredients: Vec<QIngredientOut>,
    categories: Vec<CategoryOut>,
    tags: Vec<TagOut>,
    seasons: Vec<SeasonOut>,
    author_id: i32,
    author_name: String,
    diets: Vec<DietOut>,
}

#[derive(Debug, Serialize)]
pub struct RecipeSummaryOut {
    id: i32,
    name: String,
    image: String,
    n_shares: i16,
    is_favorite: bool,
    is_private: bool,
    ingredients: Vec<QIngredientOut>,
    diets: Vec<DietOut>,
}

#[derive(Debug, Serialize)]
pub struct QIngredientOut {
    id: i32,
    name: String,
    quantity: Option<f32>,
    unit: Option<UnitOut>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeIn {
    name: String,
    notes: String,
    prep_time_min: i16,
    cook_time_min: i16,
    image: String,
    instructions: Vec<String>,
    n_shares: i16,
    is_private: bool,
    q_ingredients: Vec<QIngredientIn>,
    category_ids: Vec<i32>,
    tag_ids: Vec<i32>,
    season_ids: Vec<i32>,
    diet_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct QIngredientIn {
    id: i32,
    quantity: Option<f32>,
    unit_id: Option<i32>,
}

impl From<models::Recipe> for RecipeOut {
    fn from(r: models::Recipe) -> Self {
        Self {
            id: r.id,
            name: r.name,
            notes: r.notes,
            preparation_time_min: r.preparation_time_min,
            cooking_time_min: r.cooking_time_min,
            image: r.image,
            publication_date: r.publication_date,
            instructions: r.instructions,
            n_shares: r.n_shares,
            is_favorite: r.is_favorite,
            is_private: r.is_private,
            ingredients: r.ingredients.into_iter().map(|i| i.into()).collect(),
            categories: r.categories.into_iter().map(|c| c.into()).collect(),
            tags: r.tags.into_iter().map(|t| t.into()).collect(),
            seasons: r.seasons.into_iter().map(|s| s.into()).collect(),
            author_id: r.author_id,
            author_name: r.author_name,
            diets: r.diets.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl From<models::RecipeSummary> for RecipeSummaryOut {
    fn from(r: models::RecipeSummary) -> Self {
        Self {
            id: r.id,
            name: r.name,
            image: r.image,
            n_shares: r.n_shares,
            is_favorite: r.is_favorite,
            is_private: r.is_private,
            ingredients: r.ingredients.into_iter().map(|i| i.into()).collect(),
            diets: r.diets.into_iter().map(|d| d.into()).collect(),
        }
    }
}

impl From<models::QIngredient> for QIngredientOut {
    fn from(i: models::QIngredient) -> Self {
        Self {
            id: i.id,
            name: i.name,
            quantity: i.quantity,
            unit: i.unit.map(|u| u.into()),
        }
    }
}

impl From<QIngredientIn> for models::NewQIngredient {
    fn from(value: QIngredientIn) -> Self {
        Self {
            id: value.id,
            quantity: value.quantity,
            unit_id: value.unit_id,
        }
    }
}

impl From<RecipeIn> for models::NewRecipe {
    fn from(value: RecipeIn) -> Self {
        Self {
            name: value.name,
            notes: value.notes,
            preparation_time_min: value.prep_time_min,
            cooking_time_min: value.cook_time_min,
            image: value.image,
            instructions: value.instructions,
            n_shares: value.n_shares,
            is_private: value.is_private,
            ingredients: value.q_ingredients.into_iter().map(|i| i.into()).collect(),
            categories: value.category_ids,
            tags: value.tag_ids,
            seasons: value.season_ids,
            diets: value.diet_ids,
        }
    }
}

struct RangeVisitor;
impl<'de> Visitor<'de> for RangeVisitor {
    type Value = Range;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a strictly positive range such as 1-10")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let pair: Vec<i64> = value
            .split('-')
            .map(|s| s.parse::<i64>())
            .collect::<Result<_, _>>()
            .map_err(|e| E::custom(e))?;

        if pair.len() != 2 {
            return Err(E::custom("not a valid range format"));
        }

        Range::new(pair[0], pair[1]).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for Range {
    fn deserialize<D>(deserializer: D) -> Result<Range, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RangeVisitor)
    }
}

#[get("/recipes")]
async fn get_many_recipes(
    app: web::Data<App>,
    range_qp: web::Query<RangeQueryParams>,
    recipe_filters_qp: web::Query<RecipeFiltersQueryParams>,
    sort_method_qp: web::Query<SortMethodQueryParams>,
    user: Option<User>,
) -> impl Responder {
    let range: Range = match range_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let mut filters: RecipeFilters = match recipe_filters_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let sort_method: SortMethod = match sort_method_qp.into_inner().try_into() {
        Ok(v) => v,
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let accept_range = format!("recipe {}", app.max_recipe_fetch_size());

    if let Err(e) = app.validate_recipe_range(&range) {
        let content_range = format!("{}-{}/{}", 0, 0, app.get_recipe_count());
        let mut ret = match e {
            RangeError::OutOfBounds(..) => HttpResponse::NoContent(),
            RangeError::TooWide(..) => HttpResponse::BadRequest(),
            RangeError::FromGreaterThanTo(..) => HttpResponse::BadRequest(),
            RangeError::NotStrictlyPositive(..) => HttpResponse::BadRequest(),
        };

        return ret
            .insert_header((http::header::CONTENT_RANGE, content_range))
            .insert_header((http::header::ACCEPT_RANGES, accept_range))
            .finish();
    }

    let (recipes, total_count_filtered): (Vec<RecipeSummaryOut>, i64) = match app
        .get_many_recipes(&range, &mut filters, sort_method, user.map(|u| u.id))
        .await
    {
        Ok(v) => {
            let t = v.get(0).map(|r| r.total_count).unwrap_or(0);
            (v.into_iter().map(|x| x.into()).collect(), t)
        }
        Err(e) => {
            error!("{}", e);
            return e.into();
        }
    };

    let fetched_count = recipes.len() as i64;
    let (first_fetched, last_fetched) = match fetched_count {
        0 => (0, 0),
        _ => (range.from, range.from + fetched_count - 1),
    };

    let mut ret;
    if fetched_count < total_count_filtered && fetched_count > 0 {
        ret = HttpResponse::PartialContent();
    } else {
        ret = HttpResponse::Ok();
    }

    let content_range = format!(
        "{}-{}/{}",
        first_fetched, last_fetched, total_count_filtered
    );

    match serde_json::to_string(&recipes) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    ret.insert_header((http::header::CONTENT_RANGE, content_range))
        .insert_header((http::header::ACCEPT_RANGES, accept_range))
        .json(recipes)
}

#[post("/recipes")]
async fn add_recipe(
    new_recipe: web::Json<RecipeIn>,
    app: web::Data<App>,
    user: User,
) -> impl Responder {
    debug!("{:?}", new_recipe);

    let mut t: models::NewRecipe = new_recipe.into_inner().into();

    let new_id = match app.add_recipe(&mut t, user.id).await {
        Ok(v) => v,
        Err(e) => {
            use AppErrorWith::*;
            if let AppError(inner_e) = &e {
                error!("{}", inner_e);
            }
            return e.into();
        }
    };

    debug!("{}", new_id);
    HttpResponse::Created()
        .insert_header((http::header::LOCATION, format!("/{}", new_id)))
        .finish()
}

#[get("/recipes/{id}")]
pub async fn get_recipe(
    id: web::Path<i32>,
    app: web::Data<App>,
    user: Option<User>,
) -> impl Responder {
    let recipe: RecipeOut = match app.get_recipe(id.into_inner(), user.map(|u| u.id)).await {
        Ok(Some(v)) => v.into(),
        Ok(None) => {
            return HttpResponse::NotFound().finish();
        }
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    };

    match serde_json::to_string(&recipe) {
        Ok(json) => debug!("{}", json),
        Err(e) => error!("{}", e),
    };

    HttpResponse::Ok().json(recipe)
}

#[put("/recipes/{id}")]
async fn replace_recipe(
    id: web::Path<i32>,
    new_recipe: web::Json<RecipeIn>,
    app: web::Data<App>,
    user: User,
) -> impl Responder {
    debug!("{:?}", new_recipe);

    let mut t: models::NewRecipe = new_recipe.into_inner().into();
    match app
        .replace_recipe(id.into_inner(), &mut t, user.id, user.is_admin)
        .await
    {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            use AppErrorWith::*;
            if let AppError(inner_e) = &e {
                error!("{}", inner_e);
            }
            return e.into();
        }
    };

    HttpResponse::Ok().finish()
}

#[delete("/recipes/{id}")]
async fn delete_recipe(id: web::Path<i32>, app: web::Data<App>, _admin: Admin) -> impl Responder {
    match app.delete_recipe(id.into_inner()).await {
        Ok(Some(_)) => (),
        Ok(None) => return HttpResponse::NotFound().finish(),
        Err(e) => {
            error!("{:?}", e);
            return e.into();
        }
    }

    HttpResponse::NoContent().finish()
}
