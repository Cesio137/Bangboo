use crate::discord::app::context::PrefixCommandContext;
use crate::discord::app::creators::{create_event, EventHandler};
use crate::tools::automod::DangerLevel;
use twilight_model::gateway::event::{Event, EventType};

pub fn message_create() -> EventHandler {
    create_event(EventType::MessageCreate, |event, context| async move {
        let message = match event {
            Event::MessageCreate(message) => message,
            _ => return,
        };
        if message.author.bot { return; }

        if let Some(callback) = context.commands.lock().await.prefix_commands.get(message.content.as_str()) {
            let ctx = PrefixCommandContext::new(message, context.client).unwrap();
            callback(ctx).await;
            return
        }

        if message.guild_id.is_some() && !message.author.bot {
            let result = context.scam_filter.lock().await.filter_message(&message.content);
            match result {
                DangerLevel::Safe => {}
                DangerLevel::High => {
                    context.scam_filter.lock().await.handle_spam(context.client, message, None).await;
                    return;
                }
                DangerLevel::HighReport(report) => {
                    context.scam_filter.lock().await.handle_spam(context.client, message, Some(report)).await;
                    return;
                }
            };
        }


    })
}
