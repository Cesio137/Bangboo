use crate::discord::app::creators::{PrefixCommandHandler, ResponderHandler, SlashCommandHandler};
use crate::discord::commands::{prefix_commands, slash_commands};
use crate::discord::responders::responders;
use serenity::builder::CreateCommand;
use std::collections::HashMap;

pub struct App {
    pub commands: Vec<CreateCommand<'static>>,
    pub prefix_command_handlers: HashMap<String, Box<dyn PrefixCommandHandler + Send + Sync>>,
    pub slash_command_handlers: HashMap<String, Box<dyn SlashCommandHandler + Send + Sync>>,
    pub responder_handlers: HashMap<String, Box<dyn ResponderHandler + Send + Sync>>,
}

impl App {
    pub fn new() -> Self {
        let slash_commands = slash_commands();
        let mut commands = Vec::new();
        let mut slash_command_handlers = HashMap::new();
        for slash_command in slash_commands {
            let cmd = slash_command.command();
            let name = extract_command_name(&cmd);
            commands.push(cmd);
            slash_command_handlers.insert(name, slash_command);
        }

        let prefix_commands = prefix_commands();
        let mut prefix_command_handlers = HashMap::new();
        for command in prefix_commands {
            let name = format!("!{}", command.name());
            prefix_command_handlers.insert(name, command);
        }

        let responders = responders();
        let mut responder_handlers = HashMap::new();
        for responder in responders {
            let custom_id = responder.custom_id();
            responder_handlers.insert(custom_id, responder);
        }

        Self {
            commands,
            prefix_command_handlers,
            slash_command_handlers,
            responder_handlers,
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
