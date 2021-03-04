use serde::Deserialize;

pub mod embed;
pub mod response;

pub use response::InteractionResponseBuilder;

#[derive(Debug, Deserialize)]
pub struct Interaction {
    version: i32,
    id: String,
    token: String,
    #[serde(flatten)]
    data: InteractionTypeAndData,
}

impl Interaction {
    pub fn version(&self) -> i32 {
        self.version
    }

    pub fn interaction_id(&self) -> u64 {
        self.id.parse().expect("Invalid Interaction ID")
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    pub fn data(&self) -> &InteractionTypeAndData {
        &self.data
    }

    pub fn into_data(self) -> InteractionTypeAndData {
        self.data
    }
}

#[derive(Debug, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum InteractionTypeAndData {
    #[serde(deserialize_with = "interaction_ping")]
    Ping,
    #[serde(deserialize_with = "interaction_command")]
    ApplicationCommand {
        guild_id: String,
        channel_id: String,
        member: GuildMember,
        data: ApplicationCommandInteractionData,
    },
}

#[derive(Debug, Deserialize)]
pub struct GuildMember {
    user: User,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: chrono::DateTime<chrono::Utc>,
    premium_since: Option<chrono::DateTime<chrono::Utc>>,
    deaf: bool,
    mute: bool,
}

impl GuildMember {
    pub fn user(&self) -> &User {
        &self.user
    }

    pub fn nick(&self) -> Option<&str> {
        self.nick.as_deref()
    }

    pub fn nick_or_username(&self) -> &str {
        self.nick.as_deref().unwrap_or(&self.user.username)
    }

    pub fn roles(&self) -> Vec<u64> {
        self.roles
            .iter()
            .map(|s| s.parse())
            .collect::<Result<_, _>>()
            .expect("Invalid Role ID")
    }

    pub fn joined_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.joined_at
    }

    pub fn is_boosting(&self) -> bool {
        self.premium_since.is_some()
    }

    pub fn boosting_since(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        self.premium_since
    }

    pub fn is_deaf(&self) -> bool {
        self.deaf
    }

    pub fn is_mute(&self) -> bool {
        self.mute
    }
}

#[derive(Debug, Deserialize)]
pub struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<i32>,
    premium_type: Option<i32>,
    public_flags: Option<i32>,
}

impl User {
    pub fn id(&self) -> u64 {
        self.id.parse().expect("Invalid User ID")
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn discriminator(&self) -> &str {
        &self.discriminator
    }

    pub fn username_and_discriminator(&self) -> String {
        format!("{}#{}", self.username, self.discriminator)
    }

    pub fn avatar(&self) -> Option<&str> {
        self.avatar.as_deref()
    }

    pub fn cdn_avatar_path(&self) -> String {
        if let Some(avatar) = &self.avatar {
            let ext = if avatar.starts_with("a_") {
                "gif"
            } else {
                "png"
            };
            format!("/avatars/{}/{}.{}", self.id, avatar, ext)
        } else {
            let discriminator = self.discriminator.parse::<u32>().unwrap();
            format!("/embed/avatars/{}.png", discriminator % 5)
        }
    }

    pub fn is_bot(&self) -> bool {
        self.bot.unwrap_or(false)
    }

    pub fn is_system(&self) -> bool {
        self.system.unwrap_or(false)
    }

    pub fn is_mfa_enabled(&self) -> bool {
        self.mfa_enabled.unwrap_or(false)
    }
}

#[derive(Debug, Deserialize)]
pub struct ApplicationCommandInteractionData {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub options: Vec<ApplicationCommandInteractionDataOption>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ApplicationCommandInteractionDataOption {
    Value {
        name: String,
        value: crate::OptionValue,
    },
    Subcommand {
        name: String,
        options: Vec<ApplicationCommandInteractionDataOption>,
    },
}

fn interaction_ping<'de, D>(d: D) -> Result<(), D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Ping {
        #[serde(rename = "type")]
        ty: i32,
    }

    let ping = Ping::deserialize(d)?;
    if ping.ty == 1 {
        Ok(())
    } else {
        Err(serde::de::Error::custom("Not a Ping type"))
    }
}

fn interaction_command<'de, D>(
    d: D,
) -> Result<
    (
        String,
        String,
        GuildMember,
        ApplicationCommandInteractionData,
    ),
    D::Error,
>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct ApplicationCommand {
        #[serde(rename = "type")]
        ty: i32,
        guild_id: String,
        channel_id: String,
        member: GuildMember,
        data: ApplicationCommandInteractionData,
    }

    let ping = ApplicationCommand::deserialize(d)?;
    if ping.ty == 2 {
        Ok((ping.guild_id, ping.channel_id, ping.member, ping.data))
    } else {
        Err(serde::de::Error::custom("Not a ApplicationCommand type"))
    }
}
