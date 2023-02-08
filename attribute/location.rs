use crate::attribute::{AsAttribute, AsUnderlying};

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use url::{ParseError, Url};

#[derive(Debug, Clone)]
pub struct LocationError(ParseError);

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
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
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::location::Location;
    /// let link = "https://www.google.com/";
    /// let url = Url::parse(link).unwrap();
    /// let location = Location::new(url.clone());
    ///
    /// assert_eq!(location.as_underlying(), url);
    /// ```
    pub fn new(url: Url) -> Self {
        Self(url)
    }
}

impl AsAttribute for Location {
    type Error = LocationError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{AsAttribute, AsUnderlying};
    /// # use sitemaps::attribute::location::Location;
    /// let link = "https://www.google.com/";
    /// let url = Url::parse(link).unwrap();
    /// let location = Location::parse(link).unwrap();
    ///
    /// assert_eq!(location.as_underlying(), url);
    /// ```
    fn parse(location: &str) -> Result<Self, Self::Error> {
        let location = Url::parse(location)?;
        let location = Self::new(location);
        Ok(location)
    }
}

impl AsUnderlying<Url> for Location {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{AsAttribute, AsUnderlying};
    /// # use sitemaps::attribute::location::Location;
    /// let link = "https://www.google.com/";
    /// let url = Url::parse(link).unwrap();
    /// let location = Location::parse(link).unwrap();
    ///
    /// assert_eq!(location.as_underlying(), url);
    /// ```
    fn as_underlying(&self) -> Url {
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
        value.as_underlying()
    }
}
