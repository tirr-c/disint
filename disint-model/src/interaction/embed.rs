use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct Embed {
    pub title: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub footer: Option<Footer>,
    pub thumbnail: Option<Media>,
    pub image: Option<Media>,
    pub video: Option<Media>,
    pub provider: Option<Provider>,
    pub author: Option<Author>,
    pub fields: Option<Vec<Field>>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Footer {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Media {
    pub url: Option<String>,
    pub proxy_url: Option<String>,
    pub height: Option<u32>,
    pub width: Option<u32>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Provider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Author {
    pub name: Option<String>,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Field {
    name: String,
    value: String,
    inline: Option<bool>,
}

impl Field {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline: None,
        }
    }

    pub fn new_inline(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline: Some(true),
        }
    }
}
