use super::*;

mod state {
    #[derive(Debug)]
    pub struct NoOptions;

    #[derive(Debug)]
    pub struct SubgroupOptions {
        pub(crate) options: Vec<super::ApplicationCommandOption>,
    }

    #[derive(Debug)]
    pub struct RegularOptions {
        pub(crate) options: Vec<super::ApplicationCommandOption>,
    }
}

mod option_state {
    #[derive(Debug, Default)]
    pub struct SubcommandGroupIncomplete;

    #[derive(Debug, Default)]
    pub struct SubcommandIncomplete;

    #[derive(Debug)]
    pub struct Subcommand {
        pub(crate) options: Vec<super::ApplicationCommandOption>,
    }

    #[derive(Debug)]
    pub struct RegularIncomplete {
        pub(crate) required: bool,
    }

    impl Default for RegularIncomplete {
        fn default() -> Self {
            Self { required: true }
        }
    }

    #[derive(Debug)]
    pub struct Regular {
        pub(crate) ty: super::ApplicationCommandOptionType,
        pub(crate) required: bool,
        pub(crate) choices: Vec<super::ApplicationCommandOptionChoice>,
    }
}

#[derive(Debug)]
pub struct ApplicationCommandBuilder<State> {
    name: String,
    description: String,
    state: State,
}

impl ApplicationCommandBuilder<state::NoOptions> {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            state: state::NoOptions,
        }
    }

    pub fn subcommand(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
        f: impl FnOnce(ApplicationCommandOptionBuilder<option_state::SubcommandIncomplete>) -> ApplicationCommandOptionBuilder<option_state::Subcommand>,
    ) -> ApplicationCommandBuilder<state::SubgroupOptions> {
        let builder = ApplicationCommandOptionBuilder {
            name: name.into(),
            description: description.into(),
            state: Default::default(),
        };
        let builder = f(builder);
        let option = builder.finish();

        ApplicationCommandBuilder {
            name: self.name,
            description: self.description,
            state: state::SubgroupOptions {
                options: vec![option],
            },
        }
    }

    pub fn option(
        self,
        name: impl Into<String>,
        description: impl Into<String>,
        f: impl FnOnce(ApplicationCommandOptionBuilder<option_state::RegularIncomplete>) -> ApplicationCommandOptionBuilder<option_state::Regular>,
    ) -> ApplicationCommandBuilder<state::RegularOptions> {
        let builder = ApplicationCommandOptionBuilder {
            name: name.into(),
            description: description.into(),
            state: Default::default(),
        };
        let builder = f(builder);
        let option = builder.finish();

        ApplicationCommandBuilder {
            name: self.name,
            description: self.description,
            state: state::RegularOptions {
                options: vec![option],
            },
        }
    }
}

#[derive(Debug)]
pub struct ApplicationCommandOptionBuilder<State> {
    name: String,
    description: String,
    state: State,
}

impl ApplicationCommandOptionBuilder<option_state::Subcommand> {
    fn finish(self) -> ApplicationCommandOption {
        ApplicationCommandOption {
            ty: ApplicationCommandOptionType::SubCommand,
            name: self.name,
            description: self.description,
            required: None,
            choices_options: Some(ChoicesOrOptions::Options {
                options: self.state.options,
            }),
        }
    }
}

impl ApplicationCommandOptionBuilder<option_state::Regular> {
    pub fn required(self, required: bool) -> Self {
        Self {
            state: option_state::Regular {
                required,
                ..self.state
            },
            ..self
        }
    }

    fn finish(self) -> ApplicationCommandOption {
        let choices = if self.state.choices.is_empty() {
            None
        } else {
            Some(ChoicesOrOptions::Choices { choices: self.state.choices })
        };
        ApplicationCommandOption {
            ty: self.state.ty,
            name: self.name,
            description: self.description,
            required: Some(self.state.required),
            choices_options: choices,
        }
    }
}
