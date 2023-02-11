use crate::attribute::{AsAttribute, AsUnderlying};

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Clone)]
pub struct LastModified(OffsetDateTime);

impl LastModified {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::modified::LastModified;
    /// let time: OffsetDateTime = todo!();
    /// let location = LastModified::new(time.clone());
    ///
    /// assert_eq!(location.as_underlying(), time);
    /// ```
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl AsAttribute for LastModified {
    type Error = ParseError;

    /// Parses the attribute from the string.
    ///
    /// ```rust
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{AsAttribute, AsUnderlying};
    /// # use sitemaps::attribute::modified::LastModified;
    /// let time: OffsetDateTime = todo!();
    /// let last_modified = LastModified::parse(todo!()).unwrap();
    ///
    /// assert_eq!(last_modified.as_underlying(), time)
    /// ```
    fn parse(last_modified: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl AsUnderlying<OffsetDateTime> for LastModified {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::modified::LastModified;
    /// let time: OffsetDateTime = todo!();
    /// let last_modified = LastModified::new(time).unwrap();
    ///
    /// assert_eq!(last_modified.as_underlying(), time);
    /// ```
    fn as_underlying(&self) -> OffsetDateTime {
        self.0
    }
}

impl TryFrom<&str> for LastModified {
    type Error = ParseError;

    fn try_from(time: &str) -> Result<Self, Self::Error> {
        Self::parse(time)
    }
}

impl From<OffsetDateTime> for LastModified {
    fn from(time: OffsetDateTime) -> Self {
        Self::new(time)
    }
}

impl From<LastModified> for OffsetDateTime {
    fn from(last_modified: LastModified) -> OffsetDateTime {
        last_modified.as_underlying()
    }
}

impl Display for LastModified {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}
