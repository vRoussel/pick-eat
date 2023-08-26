mod category;
mod diet;
mod season;
mod tag;
use std::convert::TryFrom;

pub use category::*;
pub use diet::*;
pub use season::*;
pub use tag::*;

use crate::{api::APIError, storage::DBConstraint};

pub trait InvalidInput:
    std::fmt::Debug + for<'a> TryFrom<&'a DBConstraint> + Into<Vec<APIError>>
{
}

#[derive(Debug, PartialEq)]
pub enum InvalidityKind {
    AlreadyUsed,
    Empty,
}
