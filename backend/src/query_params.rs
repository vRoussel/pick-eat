use serde::de::{self, Deserializer, Visitor};
use serde::Deserialize;
use std::fmt;

#[derive(Debug)]
pub struct Range {
    pub from: i64,
    pub to: i64,
}

pub enum RangeError {
    OutOfBounds,
    TooWide,
    Invalid,
}

struct RangeVisitor;
impl<'de> Visitor<'de> for RangeVisitor {
    type Value = Range;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("A strictly positive range such as 1-10")
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
        Ok(Range {
            from: pair[0],
            to: pair[1],
        })
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

impl Range {
    pub fn validate(
        &self,
        max_range_size: Option<i64>,
        total_count: Option<i64>,
    ) -> Result<(), RangeError> {
        //TODO < 0 not working
        if self.from < 0 || self.to < 0 || self.to < self.from {
            return Err(RangeError::Invalid);
        }

        let range_size = self.to - self.from + 1;
        if max_range_size.is_some() && range_size > max_range_size.unwrap() {
            return Err(RangeError::TooWide);
        }

        if total_count.is_some() && self.from > total_count.unwrap() {
            return Err(RangeError::OutOfBounds);
        }

        Ok(())
    }
}
