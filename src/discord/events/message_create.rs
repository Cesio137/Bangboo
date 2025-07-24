use crate::discord::app::base::App;
use crate::tools::automod;
use serenity::all::Message;
use serenity::client::Context;

pub async fn run(app: &App, ctx: Context, message: Message) {
    automod::filter_message(&ctx, &message).await;
    if let Some(callback) = app.prefix_command_handlers.get(&message.content) {
        callback.run(app, ctx, message).await;
    }
}
