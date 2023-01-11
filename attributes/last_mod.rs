use crate::attributes::AsAttribute;
use std::error::Error;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct LastModifiedError;

impl Display for LastModifiedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for LastModifiedError {}

#[derive(Debug, PartialEq, Clone)]
pub struct LastModified(OffsetDateTime);

impl LastModified {
    pub fn parse(date: &str) -> Result<Self, LastModifiedError> {
        todo!()
    }

    pub fn new(date: OffsetDateTime) -> Self {
        Self(date)
    }
}

impl AsAttribute<OffsetDateTime> for LastModified {
    fn as_attribute(&self) -> OffsetDateTime {
        self.0
    }
}

impl TryFrom<&str> for LastModified {
    type Error = LastModifiedError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<LastModified> for OffsetDateTime {
    fn from(value: LastModified) -> Self {
        value.as_attribute()
    }
}

impl From<OffsetDateTime> for LastModified {
    fn from(value: OffsetDateTime) -> Self {
        Self::new(value)
    }
}

#[cfg(feature = "serde")]
impl Serialize for LastModified {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for LastModified {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
