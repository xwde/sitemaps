use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub struct Image {
    pub location: Url,
}
