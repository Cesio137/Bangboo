use twilight_model::{
    application::interaction::application_command::{CommandData, CommandDataOption},
    gateway::payload::incoming::InteractionCreate,
};

pub fn get_command_data(interaction: &Box<InteractionCreate>) -> Option<Box<CommandData>> {
    match &interaction.data {
        Some(data) => match data {
            twilight_model::application::interaction::InteractionData::ApplicationCommand(
                command_data,
            ) => {
                return command_data.clone().into();
            }
            _ => None,
        },
        None => None,
    }
}

pub fn get_options(interaction: &Box<InteractionCreate>) -> Vec<CommandDataOption> {
    match get_command_data(interaction) {
        Some(command_data) => {
            if command_data.options.len() == 0 {
                return vec![];
            }
            return command_data.clone().options;
        }
        None => vec![],
    }
}
