use crate::discord::app::creators::{create_event, EventHandler};
use crate::utils::logger::error;
use twilight_model::application::interaction::InteractionData::ApplicationCommand;
use twilight_model::gateway::event::{Event, EventType};

pub fn interaction_create() -> EventHandler {
    create_event(EventType::InteractionCreate, |event, context| async move {
        let interaction = match event {
            Event::InteractionCreate(message) => message,
            _ => return,
        };

        let data = match &interaction.data {
            Some(data) => data,
            None => {
                error(&format!("Failed to get interaction data."));
                return;
            }
        };
        match data {
            ApplicationCommand(command_data) => {
                if let Some(callback) = context.commands.lock().await.slash_commands.get(&command_data.name) {
                    (callback.reply)(interaction.clone(), context.client).await;
                }
            }
            _ => {}
        }
    })
}