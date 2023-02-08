use crate::attribute::frequency::ChangeFrequency;
use crate::attribute::location::Location;
use crate::attribute::modified::LastModified;
use crate::attribute::priority::Priority;

use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct SitemapEntry {
    pub location: Location,
    pub last_modified: Option<LastModified>,
    pub change_frequency: Option<ChangeFrequency>,
    pub priority: Option<Priority>,
}

impl SitemapEntry {
    pub fn parse(url: &str) -> Result<Self, ()> {
        let url = Url::parse(url).unwrap();
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
