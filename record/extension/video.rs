use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub enum VideoUrl {
    Content(Url),
    Player(Url),
    Both(Url, Url),
}

impl VideoUrl {
    pub fn from_content(url: Url) -> Self {
        Self::Content(url)
    }

    pub fn from_player(url: Url) -> Self {
        Self::Player(url)
    }

    pub fn from_both(content: Url, player: Url) -> Self {
        Self::Both(content, player)
    }

    pub fn content(self) -> Option<Url> {
        match self {
            Self::Content(u) => Some(u),
            Self::Player(_) => None,
            Self::Both(u, _) => Some(u),
        }
    }

    pub fn player(self) -> Option<Url> {
        match self {
            Self::Content(_) => None,
            Self::Player(u) => Some(u),
            Self::Both(_, u) => Some(u),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Video {
    title: String,
    description: String,
    link: VideoUrl,
}

impl Video {
    pub fn new(title: String, description: String, url: VideoUrl) -> Self {
        todo!()
    }
}
