pub mod public;
pub mod prefix;

use crate::discord::app::creators::{PrefixCommand, SlashCommand};
use prefix::*;
use public::*;

pub fn prefix_commands() -> Vec<PrefixCommand> {
    let mut commands = Vec::new();

    let ping = ping::command();
    commands.push(ping);

    commands
}

pub fn slash_commands() -> Vec<SlashCommand> {
    let mut commands = Vec::new();
    
    let age = age::command();
    commands.push(age);
    
    let canvas = canvas::command();
    commands.push(canvas);
    
    commands
}