mod category;
mod diet;
mod season;
mod tag;
mod unit;
use std::convert::TryFrom;
mod ingredient;

pub use category::*;
pub use diet::*;
pub use ingredient::*;
pub use season::*;
pub use tag::*;
pub use unit::*;

use crate::{api::APIError, storage::DBConstraint};

pub trait InvalidInput:
    std::fmt::Debug + for<'a> TryFrom<&'a DBConstraint, Error = String> + Into<Vec<APIError>>
{
}

#[derive(Debug, PartialEq)]
pub enum InvalidityKind {
    AlreadyUsed,
    Empty,
    InvalidRef,
    TooShort,
    BadFormat,
    Mismatch,
    Expired,
    NotFound,
}
