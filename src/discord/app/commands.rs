use crate::discord::commands::{prefix_commands, slash_commands};
use colored::Colorize;
use std::collections::HashMap;
use twilight_http::Client;
use twilight_model::{
    application::command::Command, id::{marker::ApplicationMarker, Id}
};

use super::creators::{PrefixCommandCallback, SlashCommandCallback};

pub struct AppCommands {
    pub commands: Vec<Command>,
    pub slash_commands: HashMap<String, SlashCommandCallback>,
    pub prefix_commands: HashMap<String, PrefixCommandCallback>,
}

impl AppCommands {
    pub fn new() -> Self {
        let register_commands = slash_commands();
        let mut commands = Vec::new();
        let mut slash_commands = HashMap::new();

        for (name, command) in register_commands {
            commands.push(command.command);
            slash_commands.insert(name, command.run);
        }

        let prefix_commands = prefix_commands();

        Self {
            commands,
            slash_commands,
            prefix_commands
        }
    }

    pub async fn register_slash_commands(&self, client: &Client, id: Id<ApplicationMarker>) {
        let result = client.interaction(id).set_global_commands(&self.commands).await;
        match result {
            Ok(_) => {
                for (name, _) in self.slash_commands.iter() {
                    println!(
                        "{} {} {} {}",
                        "✔".bright_green(),
                        "[/]".green(),
                        name.bright_cyan(),
                        "command loaded!".bright_green()
                    );
                }
            }
            Err(err) => {
                println!("{}", " <ERROR> ".on_red());
                for (name, _) in self.slash_commands.iter() {
                    println!(
                        "{} {} {} {}",
                        "✖".bright_red(),
                        "[/]".red(),
                        name.bright_cyan(),
                        "command not loaded!".bright_red()
                    );
                }
                println!("{} {}", " MOTIVE: ".on_red(), err.to_string());
                println!("{}", " <ERROR/> ".on_red());
            }
        }
    }

    pub fn len(&self) -> usize {
        self.slash_commands.len()
    }
}
