mod category;
mod season;
mod tag;
use std::convert::TryFrom;

pub use category::*;
pub use season::*;
pub use tag::*;

use crate::{api::APIError, storage::DBConstraint};

pub trait InvalidInput:
    std::fmt::Debug + for<'a> TryFrom<&'a DBConstraint> + Into<Vec<APIError>>
{
}

#[derive(Debug)]
pub enum InvalidityKind {
    AlreadyUsed,
    Empty,
}
