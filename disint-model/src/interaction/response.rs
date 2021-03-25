use derive_builder::Builder;
use serde::Serialize;

use super::embed::Embed;

#[derive(Debug)]
#[non_exhaustive]
pub enum InteractionResponse {
    Pong,
    ChannelMessageWithSource(ApplicationCommandCallbackData),
    DeferredChannelMessageWithSource,
}

impl Serialize for InteractionResponse {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        InteractionResponseInner::new(self).serialize(s)
    }
}

#[derive(Debug, Serialize)]
pub struct ApplicationCommandCallbackData {
    tts: Option<bool>,
    content: Option<String>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug, Default, Builder)]
#[builder(default)]
pub struct AllowedMentions {
    roles: AllowedMentionsKind,
    users: AllowedMentionsKind,
    deny_mention_everyone: bool,
}

impl Serialize for AllowedMentions {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[derive(Default, Serialize)]
        struct AllowedMentionsInner<'a> {
            parse: Vec<&'static str>,
            roles: Option<&'a [String]>,
            users: Option<&'a [String]>,
        }

        let mut inner = AllowedMentionsInner::default();
        if !self.deny_mention_everyone {
            inner.parse.push("everyone");
        }
        match &self.roles {
            AllowedMentionsKind::All => inner.parse.push("roles"),
            AllowedMentionsKind::List(l) => inner.roles = Some(&*l),
            _ => {}
        }
        match &self.users {
            AllowedMentionsKind::All => inner.parse.push("users"),
            AllowedMentionsKind::List(l) => inner.users = Some(&*l),
            _ => {}
        }

        inner.serialize(s)
    }
}

#[derive(Clone, Debug)]
pub enum AllowedMentionsKind {
    All,
    List(Vec<String>),
    None,
}

impl AllowedMentionsKind {
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::iter::FromIterator<String> for AllowedMentionsKind {
    fn from_iter<T: IntoIterator<Item = String>>(iter: T) -> Self {
        Self::List(<Vec<_> as std::iter::FromIterator<_>>::from_iter(iter))
    }
}

impl Default for AllowedMentionsKind {
    fn default() -> Self {
        Self::All
    }
}

#[derive(Serialize)]
struct InteractionResponseInner<'a> {
    #[serde(rename = "type")]
    ty: i32,
    data: Option<&'a ApplicationCommandCallbackData>,
}

impl<'a> InteractionResponseInner<'a> {
    fn new(val: &'a InteractionResponse) -> Self {
        use InteractionResponse::*;

        let (ty, data) = match val {
            Pong => (1, None),
            ChannelMessageWithSource(data) => (4, Some(data)),
            DeferredChannelMessageWithSource => (5, None),
        };

        Self { ty, data }
    }
}

#[derive(Debug)]
pub struct InteractionResponseBuilder<State>(State);

#[derive(Debug)]
pub struct Pong;

#[derive(Debug)]
pub struct Deferred;

#[derive(Debug, Default)]
pub struct ChannelMessageNoContent {
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug)]
pub struct ChannelMessage {
    content: Option<String>,
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

impl InteractionResponseBuilder<Pong> {
    pub fn pong() -> Self {
        Self(Pong)
    }

    pub fn finish(self) -> InteractionResponse {
        InteractionResponse::Pong
    }
}

impl InteractionResponseBuilder<Deferred> {
    pub fn deferred() -> Self {
        Self(Deferred)
    }

    pub fn finish(self) -> InteractionResponse {
        InteractionResponse::DeferredChannelMessageWithSource
    }
}

impl InteractionResponseBuilder<ChannelMessageNoContent> {
    pub fn channel_message() -> Self {
        Self(ChannelMessageNoContent::default())
    }

    pub fn content(self, content: impl Into<String>) -> InteractionResponseBuilder<ChannelMessage> {
        let ChannelMessageNoContent {
            tts,
            embeds,
            allowed_mentions,
        } = self.0;

        InteractionResponseBuilder(ChannelMessage {
            content: Some(content.into()),
            tts,
            embeds,
            allowed_mentions,
        })
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);
        self
    }

    pub fn embed(self, embed: Embed) -> InteractionResponseBuilder<ChannelMessage> {
        let ChannelMessageNoContent {
            tts,
            mut embeds,
            allowed_mentions,
        } = self.0;

        embeds
            .get_or_insert_with(Vec::new)
            .push(embed);

        InteractionResponseBuilder(ChannelMessage {
            content: None,
            tts,
            embeds,
            allowed_mentions,
        })
    }

    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);
        self
    }
}

impl InteractionResponseBuilder<ChannelMessage> {
    pub fn finish(self) -> InteractionResponse {
        let ChannelMessage {
            content,
            tts,
            embeds,
            allowed_mentions,
        } = self.0;

        let data = ApplicationCommandCallbackData {
            tts,
            content,
            embeds,
            allowed_mentions,
        };

        InteractionResponse::ChannelMessageWithSource(data)
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.0.content = Some(content.into());
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);
        self
    }

    pub fn embed(mut self, embed: Embed) -> Self {
        self.0
            .embeds
            .get_or_insert_with(Vec::new)
            .push(embed);
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);
        self
    }
}
