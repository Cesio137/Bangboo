use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;
use colored::Colorize;
use twilight_http::{Client, Error, Response};
use twilight_http::response::marker::ListBody;
use twilight_model::{
    id::Id,
    id::marker::ApplicationMarker,
};
use twilight_model::application::command::Command;
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
        let commands: Vec<Command> = self.slash_commands.values().map(|value| value.command.clone()).collect();
        let result = client.interaction(id).set_global_commands(&commands).await;
        match result {
            Ok(_) => {
                for cmd in commands.iter()   {
                    println!("{} {} {} {}", "✔".bright_green(), "[/]".green(), cmd.name.bright_cyan(), "command loaded!".bright_green());
                }
            }
            Err(err) => {
                println!("{}", " <ERROR> ".on_red());
                for cmd in commands.iter()   {
                    println!("{} {} {} {}", "✖".bright_red(), "[/]".red(), cmd.name.bright_cyan(), "command not loaded!".bright_red());
                }
                println!("{} {}", " MOTIVE: ".on_red(), err.to_string());
                println!("{}", " <ERROR/> ".on_red());
            }
        }
    }

    pub fn len(&self) -> usize {
        self.prefix_commands.len() + self.slash_commands.len()
    }
}
