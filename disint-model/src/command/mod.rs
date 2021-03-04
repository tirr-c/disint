use serde::{Deserialize, Serialize};

pub mod builder;

pub use builder::ApplicationCommandBuilder;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommand {
    id: Option<String>,
    application_id: Option<String>,
    name: String,
    description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    options: Vec<ApplicationCommandOption>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOption {
    #[serde(rename = "type")]
    ty: ApplicationCommandOptionType,
    name: String,
    description: String,
    required: Option<bool>,
    #[serde(flatten)]
    choices_options: Option<ChoicesOrOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[repr(u32)]
pub enum ApplicationCommandOptionType {
    SubCommand = 1,
    SubCommandGroup = 2,
    String = 3,
    Integer = 4,
    Boolean = 5,
    User = 6,
    Channel = 7,
    Role = 8,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum ChoicesOrOptions {
    Choices {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        choices: Vec<ApplicationCommandOptionChoice>,
    },
    Options {
        #[serde(skip_serializing_if = "Vec::is_empty")]
        options: Vec<ApplicationCommandOption>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationCommandOptionChoice {
    name: String,
    value: crate::OptionValue,
}
