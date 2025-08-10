use crate::discord::*;
use crate::tools::*;
use serenity::all::{Context, Message};

pub async fn run(app: &App, ctx: &Context, message: &Message) {
    if message.author.bot() {
        return;
    }

    filter_message(&ctx, &message).await;

    if let Some(callback) = app.prefix_command_handlers.get(message.content.as_str()) {
        callback.run(app, ctx, message).await;
    }
}
