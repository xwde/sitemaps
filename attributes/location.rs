use crate::attributes::AsAttribute;
use std::error::Error;
use std::fmt::{Display, Formatter};
use url::{ParseError, Url};

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct LocationError(ParseError);

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for LocationError {}

impl From<ParseError> for LocationError {
    fn from(error: ParseError) -> Self {
        Self(error)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Location(Url);

impl Location {
    pub fn parse(url: &str) -> Result<Self, LocationError> {
        let location = Url::parse(url)?;
        let location = Self::new(location);
        Ok(location)
    }

    pub fn new(url: Url) -> Self {
        Self(url)
    }
}
impl AsAttribute<Url> for Location {
    fn as_attribute(&self) -> Url {
        self.0.clone()
    }
}

impl TryFrom<&str> for Location {
    type Error = LocationError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<Url> for Location {
    fn from(value: Url) -> Self {
        Self::new(value)
    }
}

impl From<Location> for Url {
    fn from(value: Location) -> Self {
        value.as_attribute()
    }
}

#[cfg(feature = "serde")]
impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        todo!()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Location {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }
}
