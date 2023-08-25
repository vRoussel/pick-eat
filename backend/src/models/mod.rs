mod tag;
use std::convert::TryFrom;

pub use tag::*;

use crate::{api::APIError, storage::DBConstraint};

pub trait InvalidInput:
    std::fmt::Debug + for<'a> TryFrom<&'a DBConstraint> + Into<Vec<APIError>>
{
}

#[derive(Debug)]
pub enum InvalidityKind {
    AlreadyUsed,
}
