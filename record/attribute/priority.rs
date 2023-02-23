use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::num::ParseFloatError;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub struct PriorityRangeError(pub f32);

impl Display for PriorityRangeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "value {} is out of expected [-1.0, 1.0] bounds", self.0)
    }
}

impl Error for PriorityRangeError {}

#[derive(Debug, Clone, PartialEq)]
pub enum PriorityError {
    ParseError(ParseFloatError),
    RangeError(PriorityRangeError),
}

impl Display for PriorityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PriorityError::ParseError(e) => Display::fmt(&e, f),
            PriorityError::RangeError(e) => Display::fmt(&e, f),
        }
    }
}

impl From<ParseFloatError> for PriorityError {
    fn from(value: ParseFloatError) -> Self {
        PriorityError::ParseError(value)
    }
}

impl From<PriorityRangeError> for PriorityError {
    fn from(value: PriorityRangeError) -> Self {
        PriorityError::RangeError(value)
    }
}

impl Error for PriorityError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Priority(f32);

impl Priority {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{Attribute, Priority};
    /// let frequency = Priority::new(0.5f32).unwrap();
    /// assert_eq!(frequency.underlying(), 0.5f32);
    /// ```
    pub fn new(priority: f32) -> Result<Self, PriorityRangeError> {
        match priority {
            x if (-1.0..=1.0).contains(&priority) => Ok(Self(x)),
            x => Err(PriorityRangeError(x)),
        }
    }

    pub const MIN: Self = Self(-1.);
    pub const MAX: Self = Self(1.);
}

impl Attribute<f32> for Priority {
    type Error = PriorityError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{Attribute, Priority};
    /// let frequency = Priority::parse("0.5").unwrap();
    /// assert_eq!(frequency.underlying(), 0.5f32);
    /// ```
    fn parse(priority: &str) -> Result<Self, Self::Error> {
        let priority = priority.parse()?;
        let priority = Self::new(priority)?;
        Ok(priority)
    }

    /// Returns the valid underlying value of the attribute.
    ///
    /// ```rust
    /// # use sitemaps::attribute::{Attribute, Priority};
    /// let frequency = Priority::parse("0.5").unwrap();
    /// assert_eq!(frequency.build().as_str(), "0.5");
    /// ```
    fn build(&self) -> String {
        self.0.to_string()
    }

    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{Attribute, Priority};
    /// let frequency = Priority::new(0.5f32).unwrap();
    /// assert_eq!(frequency.underlying(), 0.5f32);
    /// ```
    fn underlying(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Priority {
    type Error = PriorityError;

    fn try_from(priority: &str) -> Result<Self, Self::Error> {
        Self::parse(priority)
    }
}
