mod public;
mod prefix;

use std::collections::HashMap;
use crate::discord::app::creators::{PrefixCommandCallback, SlashCommand};


type PrefixCommandMap = HashMap<String, PrefixCommandCallback>;

pub fn prefix_commands() -> HashMap<String, PrefixCommandCallback> {
    let mut commands = PrefixCommandMap::new();
    // Add more commands here...
    let ping = prefix::ping::ping_command();
    commands.insert(ping.name, ping.reply);
    
    commands
}

type SlashCommandMap = HashMap<String, SlashCommand>;

pub fn slash_commands() -> HashMap<String, SlashCommand> {
    let mut commands = SlashCommandMap::new();
    // Add more commands here...
    let age = public::age::age_command();
    commands.insert(age.command.name.clone(), age);
    
    commands
}