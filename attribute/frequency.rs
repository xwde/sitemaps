use crate::attribute::{AsAttribute, AsUnderlying};

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use time::{ext::NumericalDuration, Date};
use timext::ext::NumericCalendarDuration;

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "invalid change frequency literal")
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Eq, Clone)]
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
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// let frequency = ChangeFrequency::new("daily");
    ///
    /// assert_eq!(frequency.unwrap(), ChangeFrequency::Daily);
    /// ```
    pub fn new(frequency: &str) -> Result<Self, ParseError> {
        use ChangeFrequency::*;
        match frequency {
            "always" => Ok(Always),
            "hourly" => Ok(Hourly),
            "daily" => Ok(Daily),
            "weekly" => Ok(Weekly),
            "monthly" => Ok(Monthly),
            "yearly" => Ok(Yearly),
            "never" => Ok(Never),
            _ => Err(ParseError),
        }
    }

    /// Calculates the date when the entry becomes outdated.
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.next_date(d0).unwrap();
    ///
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
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.is_outdated(d0, d1);
    ///
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

impl AsAttribute for ChangeFrequency {
    type Error = ParseError;

    /// Parses the attribute from the string.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::AsAttribute;
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// let frequency = ChangeFrequency::parse("Daily").unwrap();
    ///
    /// assert_eq!(frequency, ChangeFrequency::Daily);
    /// ```
    fn parse(frequency: &str) -> Result<Self, Self::Error> {
        Self::new(frequency.trim().to_lowercase().as_str())
    }
}

impl AsUnderlying<&'static str> for ChangeFrequency {
    /// Returns the valid underlying value of the attribute.
    ///
    /// ``` rust
    /// # use sitemaps::attribute::AsUnderlying;
    /// # use sitemaps::attribute::frequency::ChangeFrequency;
    /// let frequency = ChangeFrequency::new("daily").unwrap();
    ///
    /// assert_eq!(frequency.as_underlying(), "daily");
    /// ```
    fn as_underlying(&self) -> &'static str {
        use ChangeFrequency::*;
        match self {
            Always => "always",
            Hourly => "hourly",
            Daily => "daily",
            Weekly => "weekly",
            Monthly => "monthly",
            Yearly => "yearly",
            Never => "never",
        }
    }
}

impl TryFrom<&str> for ChangeFrequency {
    type Error = ParseError;

    fn try_from(frequency: &str) -> Result<Self, Self::Error> {
        ChangeFrequency::new(frequency)
    }
}

impl From<ChangeFrequency> for &str {
    fn from(frequency: ChangeFrequency) -> &'static str {
        frequency.as_underlying()
    }
}
