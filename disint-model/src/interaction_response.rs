use super::*;

#[derive(Debug)]
pub struct InteractionResponseBuilder<State>(State);

#[derive(Debug)]
pub struct Pong;

#[derive(Debug, Default)]
pub struct Acknowledge {
    with_source: bool,
}

#[derive(Debug, Default)]
pub struct ChannelMessageNoContent {
    with_source: bool,
    tts: Option<bool>,
    embeds: Option<Vec<Embed>>,
    allowed_mentions: Option<AllowedMentions>,
}

#[derive(Debug)]
pub struct ChannelMessage {
    with_source: bool,
    content: String,
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

impl InteractionResponseBuilder<Acknowledge> {
    pub fn acknowledge() -> Self {
        Self(Acknowledge::default())
    }

    pub fn with_source(mut self, with_source: bool) -> Self {
        self.0.with_source = with_source;
        self
    }

    pub fn finish(self) -> InteractionResponse {
        if self.0.with_source {
            InteractionResponse::AcknowledgeWithSource
        } else {
            InteractionResponse::Acknowledge
        }
    }
}

impl InteractionResponseBuilder<ChannelMessageNoContent> {
    pub fn channel_message() -> Self {
        Self(ChannelMessageNoContent::default())
    }

    pub fn content(self, content: impl Into<String>) -> InteractionResponseBuilder<ChannelMessage> {
        let ChannelMessageNoContent {
            with_source,
            tts,
            embeds,
            allowed_mentions,
        } = self.0;

        InteractionResponseBuilder(ChannelMessage {
            with_source,
            content: content.into(),
            tts,
            embeds,
            allowed_mentions,
        })
    }

    pub fn with_source(mut self, with_source: bool) -> Self {
        self.0.with_source = with_source;
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);
        self
    }

    pub fn embed(mut self, f: impl FnOnce(&mut embed::EmbedBuilder)) -> Self {
        let mut builder = embed::EmbedBuilder::default();
        f(&mut builder);
        self.0
            .embeds
            .get_or_insert_with(Vec::new)
            .push(builder.build().unwrap());
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);
        self
    }
}

impl InteractionResponseBuilder<ChannelMessage> {
    pub fn finish(self) -> InteractionResponse {
        let ChannelMessage {
            with_source,
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

        if with_source {
            InteractionResponse::ChannelMessageWithSource(data)
        } else {
            InteractionResponse::ChannelMessage(data)
        }
    }

    pub fn with_source(mut self, with_source: bool) -> Self {
        self.0.with_source = with_source;
        self
    }

    pub fn tts(mut self, tts: bool) -> Self {
        self.0.tts = Some(tts);
        self
    }

    pub fn embed(mut self, f: impl FnOnce(&mut embed::EmbedBuilder)) -> Self {
        let mut builder = embed::EmbedBuilder::default();
        f(&mut builder);
        self.0
            .embeds
            .get_or_insert_with(Vec::new)
            .push(builder.build().unwrap());
        self
    }

    pub fn allowed_mentions(mut self, allowed_mentions: AllowedMentions) -> Self {
        self.0.allowed_mentions = Some(allowed_mentions);
        self
    }
}
