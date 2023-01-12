use crate::attributes::AsAttribute;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::num::ParseFloatError;

#[cfg(feature = "serde")]
use serde::de::Error as SerdeError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    pub fn parse(priority: &str) -> Result<Self, PriorityError> {
        let priority = priority.parse()?;
        let priority = Self::new(priority)?;
        Ok(priority)
    }

    fn new(priority: f32) -> Result<Self, RangeError> {
        match priority {
            x if (-1.0..=1.0).contains(&priority) => Ok(Priority(x)),
            x => Err(RangeError(x)),
        }
    }

    pub const MIN: Self = Self(-1.);
    pub const MAX: Self = Self(1.);
}

impl AsAttribute<f32> for Priority {
    fn as_attribute(&self) -> f32 {
        self.0
    }
}

impl TryFrom<&str> for Priority {
    type Error = PriorityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Priority::parse(value)
    }
}

impl TryFrom<f32> for Priority {
    type Error = RangeError;

    fn try_from(value: f32) -> Result<Self, Self::Error> {
        Priority::new(value)
    }
}

impl From<Priority> for f32 {
    fn from(value: Priority) -> f32 {
        value.as_attribute()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Priority {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(self.as_attribute())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Priority {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        f32::deserialize(deserializer)
            .map(Self::new)?
            .map_err(SerdeError::custom)
    }
}
