use crate::attributes::AsAttribute;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use time::{ext::NumericalDuration, Date};
use timext::ext::NumericMonthDuration;

#[cfg(feature = "serde")]
use serde::de::Error as SerdeError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct ParseError;

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "invalid change frequency literal")
    }
}

impl Error for ParseError {}

#[derive(Debug, PartialEq, Clone)]
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
    pub fn parse(frequency: &str) -> Result<Self, ParseError> {
        Self::new(frequency.trim().to_lowercase().as_str())
    }

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

    /// Calculates the date when the entry becomes outdated
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// # use sitemaps::attributes::frequency::ChangeFrequency;
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.next(d0).unwrap();
    /// assert_eq!(rs, d1)
    /// ```
    pub fn next(&self, date: Date) -> Option<Date> {
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

    /// Calculates if the entry is outdated now
    ///
    /// ```rust
    /// # use time::{Date, Month};
    /// # use sitemaps::attributes::frequency::ChangeFrequency;
    /// let d0 = Date::from_calendar_date(2022, Month::September, 12).unwrap();
    /// let d1 = Date::from_calendar_date(2022, Month::October, 12).unwrap();
    /// let rs = ChangeFrequency::Monthly.is_outdated(d0, d1);
    /// assert!(rs)
    /// ```
    pub fn is_outdated(&self, date: Date, now: Date) -> bool {
        match &self {
            ChangeFrequency::Always => true,
            ChangeFrequency::Never => false,
            _ => match self.next(date) {
                Some(next) => next <= now,
                None => unreachable!(),
            },
        }
    }
}

impl AsAttribute<&'static str> for ChangeFrequency {
    fn as_attribute(&self) -> &'static str {
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

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        ChangeFrequency::new(value)
    }
}

impl From<ChangeFrequency> for &str {
    fn from(val: ChangeFrequency) -> &'static str {
        val.as_attribute()
    }
}

#[cfg(feature = "serde")]
impl Serialize for ChangeFrequency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_attribute())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ChangeFrequency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        <&str>::deserialize(deserializer)
            .map(Self::parse)?
            .map_err(SerdeError::custom)
    }
}
