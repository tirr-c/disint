use derive_builder::Builder;
use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize, Builder)]
pub struct Embed {
    #[builder(setter(into))]
    title: Option<String>,
    #[builder(setter(into))]
    description: Option<String>,
    #[builder(setter(into))]
    url: Option<String>,
    #[builder(setter(into))]
    timestamp: Option<chrono::DateTime<chrono::Utc>>,
    footer: Option<Footer>,
    image: Option<Image>,
    video: Option<Video>,
    provider: Option<Provider>,
    author: Option<Author>,
    fields: Option<Vec<Field>>,
}

#[derive(Clone, Debug, Serialize, Builder)]
pub struct Footer {
    text: String,
    #[builder(setter(into))]
    icon_url: Option<String>,
    #[builder(setter(into))]
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Builder)]
pub struct Image {
    #[builder(setter(into))]
    url: Option<String>,
    #[builder(setter(into))]
    proxy_url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Builder)]
pub struct Video {
    #[builder(setter(into))]
    url: Option<String>,
    height: Option<u32>,
    width: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Builder)]
pub struct Provider {
    #[builder(setter(into))]
    name: Option<String>,
    #[builder(setter(into))]
    url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Builder)]
pub struct Author {
    #[builder(setter(into))]
    name: Option<String>,
    #[builder(setter(into))]
    url: Option<String>,
    #[builder(setter(into))]
    icon_url: Option<String>,
    #[builder(setter(into))]
    proxy_icon_url: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Field {
    name: String,
    value: String,
    inline: Option<bool>,
}
