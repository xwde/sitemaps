use crate::attribute::ChangeFrequency;
use crate::attribute::LastModified;
use crate::attribute::Priority;
use crate::attribute::{Location, LocationError};

use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct SitemapRecord {
    pub location: Location,
    pub last_modified: Option<LastModified>,
    pub change_frequency: Option<ChangeFrequency>,
    pub priority: Option<Priority>,
}

impl SitemapRecord {
    pub fn parse(url: &str) -> Result<Self, LocationError> {
        let url = Url::parse(url)?;
        Ok(Self::new(url))
    }

    pub fn new(url: Url) -> Self {
        Self {
            location: Location::new(url),
            last_modified: None,
            change_frequency: None,
            priority: None,
        }
    }
}

impl SitemapRecord {
    pub fn replace_last_modified(&mut self, last_modified: LastModified) -> &mut Self {
        self.last_modified = Some(last_modified);
        self
    }

    pub fn replace_change_frequency(&mut self, change_frequency: ChangeFrequency) -> &mut Self {
        self.change_frequency = Some(change_frequency);
        self
    }

    pub fn replace_priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = Some(priority);
        self
    }
}
