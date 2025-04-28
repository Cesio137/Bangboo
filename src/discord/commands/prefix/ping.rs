use async_trait::async_trait;
use serenity::all::{Context, Message};
use crate::discord::app::base::App;
use crate::discord::app::creators::PrefixCommandHandler;

pub struct Ping;

#[async_trait]
impl PrefixCommandHandler for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    async fn run(&self, app: &App, ctx: Context, message: Message) {
        message.channel_id.say(ctx.http.as_ref(), "Pong!").await;
    }
}