use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use url::{ParseError as UrlError, Url};

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub struct LocationError(UrlError);

impl Display for LocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl From<UrlError> for LocationError {
    fn from(error: UrlError) -> Self {
        Self(error)
    }
}

impl From<LocationError> for UrlError {
    fn from(error: LocationError) -> Self {
        error.0
    }
}

impl Error for LocationError {}

#[derive(Debug, Clone, PartialEq)]
pub struct Location(Url);

impl Location {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{Attribute, Location};
    /// let link = "https://www.google.com/";
    /// let url = Url::parse(link).unwrap();
    /// let location = Location::new(url.clone());
    /// assert_eq!(location.underlying(), url);
    /// ```
    pub fn new(location: Url) -> Self {
        // TODO check if fully qualified
        Self(location)
    }
}

impl Attribute<Url> for Location {
    type Error = LocationError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{Attribute, Location};
    /// let link = "https://www.google.com/";
    /// let location = Location::parse(link).unwrap();
    /// assert_eq!(location.underlying(), Url::parse(link).unwrap());
    /// ```
    fn parse(location: &str) -> Result<Self, Self::Error> {
        let location = Url::parse(location)?;
        Ok(Self::new(location))
    }

    /// Returns the string representation of the attribute.
    ///
    /// ```rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{Attribute, Location};
    /// let link = "https://www.google.com/";
    /// let location = Location::parse(link).unwrap();
    /// assert_eq!(location.build().as_str(), link);
    /// ```
    fn build(&self) -> String {
        self.0.to_string()
    }

    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use url::Url;
    /// # use sitemaps::attribute::{Attribute, Location};
    /// let link = "https://www.google.com/";
    /// let location = Location::parse(link).unwrap();
    /// assert_eq!(location.underlying(), Url::parse(link).unwrap());
    /// ```
    fn underlying(&self) -> Url {
        self.0.clone()
    }
}

impl TryFrom<&str> for Location {
    type Error = LocationError;

    fn try_from(location: &str) -> Result<Self, Self::Error> {
        Self::parse(location)
    }
}
