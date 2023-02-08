use crate::attribute::location::{Location, LocationError};
use crate::attribute::modified::LastModified;

use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct SitemapIndexEntry {
    pub location: Location,
    pub last_modified: Option<LastModified>,
}

impl SitemapIndexEntry {
    pub fn parse(url: &str) -> Result<Self, LocationError> {
        let url = Url::parse(url)?;
        Ok(Self::new(url))
    }

    pub fn new(url: Url) -> Self {
        Self {
            location: Location::new(url),
            last_modified: None,
        }
    }
}
