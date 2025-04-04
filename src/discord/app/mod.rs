pub mod commands;
pub mod context;
pub mod creators;
pub mod events;

use crate::discord::app::context::AppContext;
use crate::discord::app::{
    commands::AppCommands,
    events::AppEvents
};
use crate::settings::env::ENV_SCHEMA;
use crate::tools::automod::{DangerLevel, Report, ScamFilter};
use anyhow::anyhow;
use colored::Colorize;
use ctrlc;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData::ApplicationCommand;
use twilight_model::channel::message::MessageReferenceType;
use twilight_model::gateway::CloseFrame;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub struct App;

impl App {
    pub async fn bootstrap() -> anyhow::Result<()> {
        // Initialize the tracing subscriber.
        tracing_subscriber::fmt::init();

        let mut shard = Shard::new(
            ShardId::ONE,
            ENV_SCHEMA.bot_token.clone(),
            Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MEMBERS,
        );

        // Client context
        let context = AppContext::new()?;

        // Since we only care about new messages, make the cache only
        // cache new messages.
        let cache = DefaultInMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE)
            .build();

        // Process each event as they come in.
        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let Ok(event) = item else {
                tracing::warn!(source = ?item.unwrap_err(), "error receiving event");

                continue;
            };

            // Update the cache with the event.
            cache.update(&event);

            match event {
                Event::Ready(ready) => {
                    context.commands.lock().await.register_slash_commands(&context.client, ready.application.id)
                        .await;
                    println!("\n{} {}", "➡ Online as".green(), ready.user.name.bright_green());
                    println!("{} {} {}", "⤿".bright_green(), context.commands.lock().await.len().to_string().green(), "command(s) successfully registered globally!".green())
                }
                _ => {
                    tokio::spawn( App::handle_event(event, context.clone()) );
                }
            }
        }

        Ok(())
    }

    async fn handle_event(event: Event, context: AppContext) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(callback) = context.events.lock().await.events.get(&event.kind()) {
            callback(event, context.clone()).await;
        }

        Ok(())
    }
}
