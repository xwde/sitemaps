use crate::attribute::{AsAttribute, AsUnderlying};

use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub struct RangeError(pub f32);

impl Display for RangeError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "value {} is out of expected [-1.0, 1.0] bounds", self.0)
    }
}

impl Error for RangeError {}

#[derive(Debug, Clone)]
pub enum PriorityError {
    ParseError(ParseFloatError),
    RangeError(RangeError),
}

impl Display for PriorityError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            PriorityError::ParseError(e) => Display::fmt(&e, f),
            PriorityError::RangeError(e) => Display::fmt(&e, f),
        }
    }
}

impl Error for PriorityError {}

impl From<ParseFloatError> for PriorityError {
    fn from(value: ParseFloatError) -> Self {
        PriorityError::ParseError(value)
    }
}

impl From<RangeError> for PriorityError {
    fn from(value: RangeError) -> Self {
        PriorityError::RangeError(value)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Priority(f32);

impl Priority {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::Priority;
    /// let frequency = Priority::new(0.5f32).unwrap();
    /// assert_eq!(frequency.as_underlying(), 0.5f32);
    /// ```
    pub fn new(priority: f32) -> Result<Self, RangeError> {
        match priority {
            x if (-1.0..=1.0).contains(&priority) => Ok(Self(x)),
            x => Err(RangeError(x)),
        }
    }

    pub const MIN: Self = Self(-1.);
    pub const MAX: Self = Self(1.);
}

impl AsAttribute for Priority {
    type Error = PriorityError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{AsAttribute, AsUnderlying};
    /// # use sitemaps::attribute::Priority;
    /// let frequency = Priority::parse("0.5").unwrap();
    /// assert_eq!(frequency.as_underlying(), 0.5f32);
    /// ```
    fn parse(priority: &str) -> Result<Self, Self::Error> {
        let priority = priority.parse()?;
        let priority = Self::new(priority)?;
        Ok(priority)
    }
}

impl AsUnderlying<f32> for Priority {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::Priority;
    /// let frequency = Priority::new(0.5f32).unwrap();
    /// assert_eq!(frequency.as_underlying(), 0.5f32);
    /// ```
    fn as_underlying(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Priority {
    type Error = PriorityError;

    fn try_from(priority: &str) -> Result<Self, Self::Error> {
        Priority::parse(priority)
    }
}

impl TryFrom<f32> for Priority {
    type Error = RangeError;

    fn try_from(priority: f32) -> Result<Self, Self::Error> {
        Priority::new(priority)
    }
}

impl From<Priority> for f32 {
    fn from(priority: Priority) -> f32 {
        priority.as_underlying()
    }
}

impl Display for Priority {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}
