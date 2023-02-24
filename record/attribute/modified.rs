use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::num::NonZeroU8;

use time::error::Parse;
use time::format_description::well_known::iso8601::{Config, EncodedConfig, TimePrecision};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use timext::error::InParse;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct LastModified(OffsetDateTime);

impl LastModified {
    const CONFIG: EncodedConfig = Config::DEFAULT
        .set_time_precision(TimePrecision::Second {
            decimal_digits: NonZeroU8::new(2),
        })
        .encode();

    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{Attribute, LastModified};
    /// let timestamp = OffsetDateTime::parse(
    ///     "1997-07-16T19:20:30.45+01:00",
    ///     &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::new(timestamp.clone());
    /// assert_eq!(location.underlying(), timestamp);
    /// ```
    pub fn new(time: OffsetDateTime) -> Self {
        Self(time)
    }
}

impl Display for LastModified {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ```rust
    /// # use sitemaps::attribute::{Attribute, LastModified};
    ///
    /// let raw = "1997-07-16T19:20:30.45+01:00";
    /// let modified = LastModified::parse(raw).unwrap();
    /// assert_eq!(modified.to_string(), raw)
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        // TODO use InOffsetDateTime instead
        let format = &Iso8601::<{ Self::CONFIG }>;
        let format = self.0.format(format).unwrap();
        Display::fmt(format.as_str(), f)
    }
}

impl Attribute<OffsetDateTime> for LastModified {
    type Error = LastModifiedError;

    /// Parses the attribute from the string.
    ///
    /// ```rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{Attribute, LastModified};
    ///
    /// let raw = "1997-07-16T19:20:30.45+01:00";
    /// let timestamp = OffsetDateTime::parse(
    ///     raw, &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::parse(raw).unwrap();
    /// assert_eq!(location.underlying(), timestamp);
    /// ```
    fn parse(last_modified: &str) -> Result<Self, Self::Error> {
        // TODO use InOffsetDateTime & .into_complete() instead
        let parsable = &Iso8601::DEFAULT;
        let last_modified = OffsetDateTime::parse(last_modified, parsable)?;
        Ok(Self::new(last_modified))
    }

    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use time::format_description::well_known::Iso8601;
    /// # use time::OffsetDateTime;
    /// # use sitemaps::attribute::{Attribute, LastModified};
    /// let timestamp = OffsetDateTime::parse(
    ///     "1997-07-16T19:20:30.45+01:00",
    ///     &Iso8601::DEFAULT
    /// ).unwrap();
    ///
    /// let location = LastModified::new(timestamp.clone());
    /// assert_eq!(location.underlying(), timestamp);
    /// ```
    fn underlying(&self) -> OffsetDateTime {
        self.0
    }
}

impl TryFrom<&str> for LastModified {
    type Error = LastModifiedError;

    fn try_from(last_modified: &str) -> Result<Self, Self::Error> {
        Self::parse(last_modified)
    }
}
