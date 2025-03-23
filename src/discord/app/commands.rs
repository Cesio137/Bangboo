use std::collections::HashMap;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    id::Id,
    id::marker::ApplicationMarker,
};
use crate::discord::app::creators::{PrefixCommandCallback, SlashCommand};
use crate::discord::commands::{prefix_commands, slash_commands};

pub struct AppCommands {
    pub prefix_commands: HashMap<String, PrefixCommandCallback>,
    pub slash_commands: HashMap<String, SlashCommand>,
}

impl AppCommands {
    pub fn new() -> Self {
        let prefix_commands = prefix_commands();
        let slash_commands = slash_commands();
        Self {
            prefix_commands,
            slash_commands
        }
    }

    pub async fn register_slash_commands(&self, client: Arc<Client>, id: Id<ApplicationMarker>) {
        let slash_commands = slash_commands();
        let cmd: Vec<Command> = slash_commands.values()
            .map(|slash_command| slash_command.command.clone())
            .collect();

        let result = client.interaction(id).set_global_commands(&cmd).await;
        if let Err(err) = result {
            eprint!("Error trying to register slash commands: {:?}", err)
        }
    }
}


