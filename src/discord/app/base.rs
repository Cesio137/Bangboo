use crate::discord::app::creators::{PrefixCommandHandler, SlashCommandHandler};
use crate::discord::interaction::commands::{prefix_commands, slash_commands};
use crate::tools::automod::ScamFilter;
use serenity::builder::CreateCommand;
use std::collections::HashMap;

pub struct App {
    pub commands: Vec<CreateCommand>,
    pub prefix_command_handlers: HashMap<String, Box<dyn PrefixCommandHandler + Send + Sync>>,
    pub slash_command_handlers: HashMap<String, Box<dyn SlashCommandHandler + Send + Sync>>,
    pub scam_filter: ScamFilter
}

impl App {
    pub fn new() -> Self {
        let slash_commands = slash_commands();
        let mut commands = Vec::new();
        let mut slash_command_handlers = HashMap::new();
        for slash_command in slash_commands {
            let command_handler = slash_command;
            let name = extract_command_name(&command_handler.command());
            commands.push(command_handler.command());
            slash_command_handlers.insert(name, command_handler);
        }
        
        let prefix_commands = prefix_commands();
        let mut prefix_command_handlers = HashMap::new();
        for command in prefix_commands {
            let command_handler = command;
            let name = format!("!{}", command_handler.name());
            prefix_command_handlers.insert(name, command_handler);
        }
        let scam_filter = ScamFilter::new().unwrap();
        Self {
            commands,
            prefix_command_handlers,
            slash_command_handlers,
            scam_filter
        }
    }
}

fn extract_command_name(command: &CreateCommand) -> String {
    let serialized = serde_json::to_value(command).unwrap_or_default();

    if let Some(name) = serialized.get("name").and_then(|n| n.as_str()) {
        return name.to_string();
    }

    "unknown_command".to_string()
}
