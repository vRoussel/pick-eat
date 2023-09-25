use core::fmt;
use std::fmt::Display;

use thiserror::Error;

use crate::api::query_params::QueryParamError;

#[derive(Debug, Clone)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

#[derive(Error, Debug)]
pub enum RangeError {
    #[error("range ({0}) starts after the max count ({1})")]
    OutOfBounds(Range, i64),
    #[error("range ({0}) is wider than allowed ({1})")]
    TooWide(Range, i64),
    #[error("from ({0}) should be less than or equal to to ({1}) in range")]
    FromGreaterThanTo(i64, i64),
    #[error("both from ({0}) and to ({1}) should be strictly positive in range")]
    NotStrictlyPositive(i64, i64),
}

impl From<RangeError> for QueryParamError {
    fn from(value: RangeError) -> Self {
        Self::WrongValue {
            param: "range",
            error: value.to_string(),
        }
    }
}

impl Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.from, self.to)
    }
}

impl Range {
    pub fn new(from: i64, to: i64) -> Result<Self, RangeError> {
        if from <= 0 || to <= 0 {
            return Err(RangeError::NotStrictlyPositive(from, to));
        }
        if from > to {
            return Err(RangeError::FromGreaterThanTo(from, to));
        }
        Ok(Self { from, to })
    }
    pub fn validate(
        &self,
        max_range_size: Option<i64>,
        total_count: Option<i64>,
    ) -> Result<(), RangeError> {
        let range_size = self.to - self.from + 1;
        if let Some(v) = max_range_size {
            if range_size > v {
                return Err(RangeError::TooWide(self.to_owned(), v));
            }
        }

        if let Some(v) = total_count {
            if self.from > v {
                return Err(RangeError::OutOfBounds(self.to_owned(), v));
            }
        }

        Ok(())
    }
}
