use crate::discord::app::context::AppContext;
use crate::discord::commands::prefix_commands;
use crate::tools::automod::DangerLevel;
use anyhow::Result;
use std::sync::Arc;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn event(message: Box<MessageCreate>, context: Arc<AppContext>) -> Result<()> {
    if message.author.bot { return Ok(()); }

    if message.guild_id.is_some() && !message.author.bot {
        let result = context.scam_filter.filter_message(&message.content);
        match result {
            DangerLevel::Safe => {}
            DangerLevel::High => {
                context.scam_filter.handle_spam(&context.client, message, None).await?;
                return Ok(());
            }
            DangerLevel::HighReport(report) => {
                context.scam_filter.handle_spam(&context.client, message, Some(report)).await?;
                return Ok(());
            }
        }
    }

    prefix_commands(&message.content, message.clone(), Arc::clone(&context.client)).await?;

    Ok(())
}
