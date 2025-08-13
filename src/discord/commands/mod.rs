pub mod prefix;
pub mod public;

use crate::discord::*;
use public::*;

pub fn prefix_commands() -> Vec<Box<dyn PrefixCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn PrefixCommandHandler + Send + Sync>> = vec![
        //Box::new(prefix::canvas::Canvas),
    ];

    commands
}

pub fn slash_commands() -> Vec<Box<dyn SlashCommandHandler + Send + Sync>> {
    let commands: Vec<Box<dyn SlashCommandHandler + Send + Sync>> = vec![
        Box::new(Age),
        Box::new(Discloud),
        Box::new(Fab),
        Box::new(Moderate),
        Box::new(Prompt),
        Box::new(Social),
    ];

    commands
}
