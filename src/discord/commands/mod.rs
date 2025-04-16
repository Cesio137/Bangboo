mod prefix;
mod public;

use super::app::creators::{PrefixCommandCallback, SlashCommand};
use std::collections::HashMap;

type PrefixCommandMap = HashMap<String, PrefixCommandCallback>;

pub fn prefix_commands() -> HashMap<String, PrefixCommandCallback> {
    let mut commands = PrefixCommandMap::new();
    // Add more commands here...
    let ping = prefix::ping::ping_command();
    commands.insert(ping.name, ping.run);

    commands
}

type SlashCommandMap = HashMap<String, SlashCommand>;

pub fn slash_commands() -> HashMap<String, SlashCommand> {
    let mut commands = SlashCommandMap::new();
    // Debug commands
    if cfg!(debug_assertions) {
        let canvas = public::canvas::canvas_command();
        commands.insert(canvas.command.name.clone(), canvas);
    }
    // Add more commands here...
    let age = public::age::age_command();
    commands.insert(age.command.name.clone(), age);
    
    commands
}
