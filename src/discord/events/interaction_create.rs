use crate::discord::app::context::AppContext;
use anyhow::{anyhow, Result};
use std::sync::Arc;
use twilight_model::{
    application::interaction::InteractionData::ApplicationCommand,
    gateway::payload::incoming::InteractionCreate
};

pub async fn run(interaction: Box<InteractionCreate>, context: Arc<AppContext>) -> Result<()> {
    let data = match &interaction.data {
        Some(data) => data,
        None => return Err(anyhow!("Failed to get interaction data.")),
    };
    match data {
        ApplicationCommand(command_data) => {
            if let Some(callback) = context.commands.slash_commands.get(&command_data.name) {
                callback(interaction.clone(), Arc::clone(&context.client)).await?;
            }
        }
        _ => {}
    }

    Ok(())
}