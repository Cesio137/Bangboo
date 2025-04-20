use crate::discord::app::creators::{PrefixCommandCallback, SlashCommandCallback};
use crate::discord::interaction::commands::{prefix_commands, slash_commands};
use crate::tools::automod::ScamFilter;
use serenity::builder::CreateCommand;
use std::collections::HashMap;

pub struct App {
    pub commands: Vec<CreateCommand>,
    pub prefix_commands: HashMap<String, PrefixCommandCallback>,
    pub slash_commands: HashMap<String, SlashCommandCallback>,
    pub scam_filter: ScamFilter
}

unsafe impl Send for App {}
unsafe impl Sync for App {}

impl App {
    pub fn new() -> Self {
        let cmds = slash_commands();
        let mut commands = Vec::new();
        let mut slash_commands_callback = HashMap::new();
        for cmd in cmds {
            let name = extract_command_name(&cmd.command);
            commands.push(cmd.command);
            slash_commands_callback.insert(name, cmd.run);
        }
        
        let mut prefix_commands_callback = HashMap::new();
        let prefix_commands = prefix_commands();
        for command in prefix_commands {
            prefix_commands_callback.insert(command.name, command.run);
        }
        let scam_filter = ScamFilter::new().unwrap();
        Self {
            commands,
            prefix_commands: prefix_commands_callback,
            slash_commands: slash_commands_callback,
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
