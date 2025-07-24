pub mod prefix;
pub mod public;

use crate::discord::app::creators::{PrefixCommandHandler, SlashCommandHandler};
use public::*;
use crate::discord::commands::prefix::canvas;

pub fn prefix_commands() -> Vec<Box<dyn PrefixCommandHandler + Send + Sync>> {
    let mut commands: Vec<Box<dyn PrefixCommandHandler + Send + Sync>> = Vec::new();

    commands.push(Box::new(canvas::Canvas));

    commands
}

pub fn slash_commands() -> Vec<Box<dyn SlashCommandHandler + Send + Sync>> {
    let mut commands: Vec<Box<dyn SlashCommandHandler + Send + Sync>> = Vec::new();

    commands.push(Box::new(age::Age));

    commands
}
