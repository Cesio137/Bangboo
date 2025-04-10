use crate::discord::commands::commands;
use colored::Colorize;
use twilight_http::Client;
use twilight_model::{
    application::command::Command,
    id::marker::ApplicationMarker,
    id::Id
};

pub struct AppCommands {
    pub slash_commands: Vec<Command>,
}

impl AppCommands {
    pub fn new() -> Self {
        let slash_commands = commands();
        Self {
            slash_commands,
        }
    }

    pub async fn register_slash_commands(&self, client: &Client, id: Id<ApplicationMarker>) {
        let result = client.interaction(id).set_global_commands(&self.slash_commands).await;
        match result {
            Ok(_) => {
                for cmd in self.slash_commands.iter() {
                    println!(
                        "{} {} {} {}",
                        "✔".bright_green(),
                        "[/]".green(),
                        cmd.name.bright_cyan(),
                        "command loaded!".bright_green()
                    );
                }
            }
            Err(err) => {
                println!("{}", " <ERROR> ".on_red());
                for cmd in self.slash_commands.iter() {
                    println!(
                        "{} {} {} {}",
                        "✖".bright_red(),
                        "[/]".red(),
                        cmd.name.bright_cyan(),
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
