use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use time::error::Parse;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use timext::error::InParse;

use crate::attribute::{AsAttribute, AsUnderlying};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LastModifiedError {
    CompleteTimestamp(Parse),
    InCompleteTimestamp(InParse),
}

impl Display for LastModifiedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::CompleteTimestamp(e) => e.fmt(f),
            Self::InCompleteTimestamp(e) => e.fmt(f),
        }
    }
}

impl From<Parse> for LastModifiedError {
    fn from(error: Parse) -> Self {
        Self::CompleteTimestamp(error)
    }
}

impl From<InParse> for LastModifiedError {
    fn from(error: InParse) -> Self {
        Self::InCompleteTimestamp(error)
    }
}

impl Error for LastModifiedError {}

#[derive(Debug, PartialEq, Clone)]
pub struct LastModified(OffsetDateTime);

impl LastModified {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{AsUnderlying, LastModified};
    /// let timestamp = OffsetDateTime::parse(
    ///     "1997-07-16T19:20:30.45+01:00",
    ///     &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::new(timestamp.clone());
    /// assert_eq!(location.as_underlying(), timestamp);
    /// ```
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl AsAttribute for LastModified {
    type Error = LastModifiedError;

    /// Parses the attribute from the string.
    ///
    /// ```rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{AsAttribute, AsUnderlying};
    /// # use sitemaps::attribute::LastModified;
    ///
    /// let raw = "1997-07-16T19:20:30.45+01:00";
    /// let timestamp = OffsetDateTime::parse(
    ///     raw, &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::parse(raw).unwrap();
    /// assert_eq!(location.as_underlying(), timestamp);
    /// ```
    fn parse(last_modified: &str) -> Result<Self, Self::Error> {
        // TODO use InOffsetDateTime & .into_complete() instead
        let parsable = &Iso8601::DEFAULT;
        let last_modified = OffsetDateTime::parse(last_modified, parsable)?;
        Ok(Self::new(last_modified))
    }
}

impl AsUnderlying<OffsetDateTime> for LastModified {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{AsUnderlying, LastModified};
    /// let timestamp = OffsetDateTime::parse(
    ///     "1997-07-16T19:20:30.45+01:00",
    ///     &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::new(timestamp.clone());
    /// assert_eq!(location.as_underlying(), timestamp);
    /// ```
    fn as_underlying(&self) -> OffsetDateTime {
        self.0
    }
}

impl TryFrom<&str> for LastModified {
    type Error = LastModifiedError;

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
    ///
    ///
    /// ```rust
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO use InOffsetDateTime instead
        write!(f, "{}", self.0.format(&Iso8601::DEFAULT).unwrap())
    }
}
