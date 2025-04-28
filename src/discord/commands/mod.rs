pub mod public;
pub mod prefix;

use crate::discord::app::creators::{PrefixCommandHandler, SlashCommandHandler};
use prefix::*;
use public::*;

pub fn prefix_commands() -> Vec<Box<dyn PrefixCommandHandler + Send + Sync>> {
    let mut commands: Vec<Box<dyn PrefixCommandHandler + Send + Sync>> = Vec::new();

    commands.push(Box::new(ping::Ping));

    commands
}

pub fn slash_commands() -> Vec<Box<dyn SlashCommandHandler + Send + Sync>> {
    let mut commands: Vec<Box<dyn SlashCommandHandler + Send + Sync>> = Vec::new();

    if cfg!(debug_assertions) {
        commands.push(Box::new(canvas::Canvas));
    }
    commands.push(Box::new(age::Age));
    commands.push(Box::new(moderate::Moderate));
    
    commands
}