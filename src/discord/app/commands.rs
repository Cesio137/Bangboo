use std::collections::HashMap;
use std::sync::Arc;
use colored::Colorize;
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

    pub async fn register_slash_commands(&mut self, client: Arc<Client>, id: Id<ApplicationMarker>) {
        let mut commands_to_remove = Vec::new();
        for (key, value) in self.slash_commands.iter() {
            let command = value.command.clone();
            let result = client.interaction(id).set_global_commands(&[command]).await;
            if let Err(err) = result {
                println!("{}", " <ERROR> ".on_red());
                println!("{} {} {} {}", "✖".bright_red(), "[/]".red(), key.bright_cyan(), "command not loaded!".bright_red());
                println!("{} {}", " MOTIVE: ".on_red(), err);
                println!("{}", " <ERROR/> ".on_red());                
                commands_to_remove.push(key.clone());
            } else {
                println!("{} {} {} {}", "✔".bright_green(), "[/]".green(), key.bright_cyan(), "command loaded!".bright_green());
            }
        }
        for key in commands_to_remove {
            self.slash_commands.remove(&key);
        }
        
    }

    pub fn len(&self) -> usize {
        self.prefix_commands.len() + self.slash_commands.len()
    }
}
