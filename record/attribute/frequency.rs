use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use time::{ext::NumericalDuration, Date};
use timext::ext::NumericCalendarDuration;

use crate::attribute::Attribute;

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeFrequencyError;

impl Display for ChangeFrequencyError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "invalid change frequency literal")
    }
}

impl Error for ChangeFrequencyError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeFrequency {
    Always,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Never,
}

impl ChangeFrequency {
    /// Creates the attribute from the valid underlying value.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::ChangeFrequency;
    /// let frequency = ChangeFrequency::new("daily");
    /// assert_eq!(frequency.unwrap(), ChangeFrequency::Daily);
    /// ```
    pub fn new(frequency: &str) -> Result<Self, ChangeFrequencyError> {
        use ChangeFrequency::*;
        match frequency {
            "always" => Ok(Always),
            "hourly" => Ok(Hourly),
            "daily" => Ok(Daily),
            "weekly" => Ok(Weekly),
            "monthly" => Ok(Monthly),
            "yearly" => Ok(Yearly),
            "never" => Ok(Never),
            _ => Err(ChangeFrequencyError),
        }
    }

    /// Calculates the date when the entry becomes outdated.
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// # use sitemaps::attribute::{ChangeFrequency};
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.next_date(d0).unwrap();
    /// assert_eq!(rs, d1)
    /// ```
    pub fn next_date(&self, date: Date) -> Option<Date> {
        use ChangeFrequency::*;
        match &self {
            Always | Never => None,
            Hourly => Some(date + 1.hours()),
            Daily => Some(date + 1.days()),
            Weekly => Some(date + 7.days()),
            Monthly => Some(date + 1.months()),
            Yearly => Some(date + 1.years()),
        }
    }

    /// Calculates if the entry is currently outdated.
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// # use sitemaps::attribute::ChangeFrequency;
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.is_outdated(d0, d1);
    /// assert!(rs)
    /// ```
    pub fn is_outdated(&self, date: Date, now: Date) -> bool {
        match &self {
            ChangeFrequency::Always => true,
            ChangeFrequency::Never => false,
            _ => match self.next_date(date) {
                Some(next) => next <= now,
                None => unreachable!(),
            },
        }
    }
}

impl Display for ChangeFrequency {
    /// Returns the string representation of the attribute.
    ///
    /// ```rust
    /// # use sitemaps::attribute::{Attribute, ChangeFrequency};
    /// let frequency = ChangeFrequency::new("daily").unwrap();
    /// assert_eq!(frequency.to_string(), "daily");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.underlying(), f)
    }
}

impl Attribute<&'static str> for ChangeFrequency {
    type Error = ChangeFrequencyError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{Attribute, ChangeFrequency};
    /// let frequency = ChangeFrequency::parse("Daily").unwrap();
    /// assert_eq!(frequency, ChangeFrequency::Daily);
    /// ```
    fn parse(frequency: &str) -> Result<Self, Self::Error> {
        Self::new(frequency.trim().to_lowercase().as_str())
    }

    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::{Attribute, ChangeFrequency};
    /// let frequency = ChangeFrequency::new("daily").unwrap();
    /// assert_eq!(frequency.underlying(), "daily");
    /// ```
    fn underlying(&self) -> &'static str {
        match self {
            Self::Always => "always",
            Self::Hourly => "hourly",
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::Never => "never",
        }
    }
}

impl TryFrom<&str> for ChangeFrequency {
    type Error = ChangeFrequencyError;

    fn try_from(frequency: &str) -> Result<Self, Self::Error> {
        Self::parse(frequency)
    }
}
