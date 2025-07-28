pub mod prefix;
pub mod public;

use serenity::all::FullEvent;
use crate::discord::app::creators::{PrefixCommandHandler, SlashCommandHandler};

pub fn prefix_commands() -> Vec<Box<dyn PrefixCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn PrefixCommandHandler + Send + Sync>> = vec![
        //Box::new(prefix::canvas::Canvas),
    ];

    commands
}

pub fn slash_commands() -> Vec<Box<dyn SlashCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn SlashCommandHandler + Send + Sync>> = vec![
        Box::new(public::age::Age),
        Box::new(public::discloud::Discloud),
        Box::new(public::fab::Fab),
        Box::new(public::moderate::Moderate),
        Box::new(public::prompt::Prompt),
        Box::new(public::social::Social),
    ];

    commands
}
