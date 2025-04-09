use crate::discord::app::context::AppContext;
use crate::tools::automod::DangerLevel;
use std::sync::Arc;
use twilight_model::gateway::payload::incoming::MessageCreate;
use crate::discord::commands::prefix_commands;

pub async fn event(message: Box<MessageCreate>, context: Arc<AppContext>) {
    if message.author.bot {
        return;
    }

    if prefix_commands(&message.content, message.clone(), Arc::clone(&context.client)).await.is_some() {
        return;
    };

    if message.guild_id.is_some() && !message.author.bot {
        let result = context.scam_filter.filter_message(&message.content);
        match result {
            DangerLevel::Safe => {}
            DangerLevel::High => {
                context
                    .scam_filter
                    .handle_spam(&context.client, message, None)
                    .await;
                return;
            }
            DangerLevel::HighReport(report) => {
                context
                    .scam_filter
                    .handle_spam(&context.client, message, Some(report))
                    .await;
                return;
            }
        };
    }
}
