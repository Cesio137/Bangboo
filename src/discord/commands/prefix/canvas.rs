use crate::discord::*;
use crate::utils::*;
use async_trait::async_trait;
use serenity::all::{Context, Message};

pub struct Canvas;

#[async_trait]
impl PrefixCommandHandler for Canvas {
    fn name(&self) -> String {
        "canvas".to_string()
    }

    async fn run(&self, app: &App, ctx: &Context, message: &Message) {
        let member = message.member(&ctx.http).await.unwrap();
        let user = member.user.clone();
        global_message(
            &ctx,
            &message.channel_id.expect_channel(),
            EventType::MemberAdd,
            Some(&member),
            &user,
        )
        .await;
    }
}
