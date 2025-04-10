pub mod commands;
pub mod context;

use crate::discord::app::{
    commands::AppCommands,
    context::AppContext
};
use crate::settings::env::ENV_SCHEMA;
use colored::Colorize;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tokio::signal;
use twilight_gateway::{Config, Event, EventTypeFlags, Intents, Shard, StreamExt as _};
use twilight_model::gateway::CloseFrame;

use super::events::app_events;

pub struct App;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

impl App {
    pub async fn bootstrap() -> anyhow::Result<()> {
        // Initialize the tracing subscriber.
        tracing_subscriber::fmt::init();

        let context = Arc::new(AppContext::new()?);
        let config = Config::new(
            ENV_SCHEMA.bot_token.clone(),
            Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MEMBERS
        );

        let shards =
            twilight_gateway::create_recommended(&context.client, config, |_, builder| builder.build()).await?;
        let mut senders = Vec::with_capacity(shards.len());
        let mut tasks = Vec::with_capacity(shards.len());

        for shard in shards {
            senders.push(shard.sender());
            tasks.push(tokio::spawn(App::runner(shard, Arc::clone(&context))));
        }

        signal::ctrl_c().await?;
        SHUTDOWN.store(true, Ordering::Relaxed);
        for sender in senders {
            // Ignore error if shard's already shutdown.
            _ = sender.close(CloseFrame::NORMAL);
        }

        for jh in tasks {
            _ = jh.await;
        }

        println!("\n👋 bye");

        Ok(())
    }

    async fn runner(mut shard: Shard, context: Arc<AppContext>) {
        while let Some(item) = shard.next_event(EventTypeFlags::all()).await {
            let event = match item {
                Ok(Event::GatewayClose(_)) if SHUTDOWN.load(Ordering::Relaxed) => break,
                Ok(event) => event,
                Err(source) => {
                    tracing::warn!(?source, "error receiving event");
                    continue;
                }
            };

            match event {
                Event::Ready(ready) => {
                    let commands = AppCommands::new();
                    commands.register_slash_commands(&context.client, ready.application.id).await;
                    println!("\n{} {}", "➡ Online as".green(), ready.user.name.bright_green());
                    println!("{} {} {}", "⤿".bright_green(), commands.len().to_string().green(), "command(s) successfully registered globally!".green());
                }
                _ => {
                    let ctx = Arc::clone(&context);
                    tokio::spawn(async move {
                        if let Err(error) = app_events(event, ctx).await {
                            tracing::warn!(?error, "error processing event");
                        };
                    });
                }
            }
            // You'd normally want to spawn a new tokio task for each event and
            // handle the event there to not block the shard.
            //tracing::debug!(?event, shard = ?shard.id(), "received event");
        }
    }
}
