pub mod commands;
pub mod creators;
pub mod events;

use crate::discord::app::{
    commands::AppCommands,
    events::AppEvents
};
use crate::settings::env::ENV_SCHEMA;
use colored::Colorize;
use ctrlc;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;
use twilight_cache_inmemory::{DefaultInMemoryCache, ResourceType};
use twilight_gateway::{Event, EventTypeFlags, Intents, Shard, ShardId, StreamExt as _};
use twilight_http::Client as HttpClient;
use twilight_model::application::interaction::InteractionData::ApplicationCommand;

pub struct App;

impl App {
    pub async fn bootstrap() -> anyhow::Result<()> {
        // Initialize the tracing subscriber.
        tracing_subscriber::fmt::init();

        // Use intents to only receive guild message events.
        let mut shard = Shard::new(
            ShardId::ONE,
            ENV_SCHEMA.bot_token.clone(),
            Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MEMBERS,
        );

        // HTTP is separate from the gateway, so create a new client.
        let client = Arc::new(HttpClient::new(ENV_SCHEMA.bot_token.clone()));

        // Since we only care about new messages, make the cache only
        // cache new messages.
        let cache = DefaultInMemoryCache::builder()
            .resource_types(ResourceType::MESSAGE)
            .build();

        let commands = Arc::new(Mutex::new(AppCommands::new()));
        let events = Arc::new(Mutex::new(AppEvents::new()));

        let _ = ctrlc::set_handler(move || {
            println!("\n👋 bye!");
            std::process::exit(0);
        });

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
                    commands
                        .lock()
                        .await
                        .register_slash_commands(Arc::clone(&client), ready.application.id)
                        .await;
                    println!(
                        "\n{} {}",
                        "➡ Online as".green(),
                        ready.user.name.bright_green()
                    );
                    println!(
                        "{} {} {}",
                        "⤿".bright_green(),
                        commands.lock().await.len().to_string().green(),
                        "command(s) successfully registered globally!".green()
                    )
                }
                _ => {
                    tokio::spawn(App::handle_event(
                        event,
                        Arc::clone(&client),
                        Arc::clone(&commands),
                        Arc::clone(&events),
                    ));
                }
            }
        }

        Ok(())
    }

    async fn handle_event(
        event: Event,
        client: Arc<HttpClient>,
        commands: Arc<Mutex<AppCommands>>,
        events: Arc<Mutex<AppEvents>>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(callback) = events.lock().await.events.get(&event.kind()) {
            callback(event, client).await;
            return Ok(());
        }
        match event {
            Event::MessageCreate(msg) => {
                if let Some(callback) = commands
                    .lock()
                    .await
                    .prefix_commands
                    .get(msg.content.as_str())
                {
                    callback(msg.clone(), Arc::clone(&client)).await;
                }
            }
            Event::InteractionCreate(interaction) => {
                let data = match &interaction.data {
                    Some(data) => data,
                    None => {
                        unreachable!()
                    }
                };
                match data {
                    ApplicationCommand(command_data) => {
                        if let Some(callback) =
                            commands.lock().await.slash_commands.get(&command_data.name)
                        {
                            (callback.reply)(interaction.clone(), Arc::clone(&client)).await;
                        }
                    }
                    _ => {}
                }
            }
            // Other events here...
            _ => {}
        }

        Ok(())
    }
}
