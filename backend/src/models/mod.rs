mod category;
mod diet;
mod season;
mod tag;
mod unit;
use std::convert::TryFrom;
mod account;
mod credentials;
mod ingredient;
mod range;
mod recipe;
mod token;

pub use account::*;
pub use category::*;
pub use credentials::*;
pub use diet::*;
pub use ingredient::*;
pub use range::*;
pub use recipe::*;
pub use season::*;
pub use tag::*;
pub use token::*;
pub use unit::*;

use crate::{api::errors::APIError, storage::DBConstraint};

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
    TooLong,
    BadFormat,
    Mismatch,
    Expired,
    NotFound,
    BadValue,
}
