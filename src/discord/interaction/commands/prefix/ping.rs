use crate::discord::app::creators::{create_prefix_command, PrefixCommand};

pub fn command() -> PrefixCommand {
    create_prefix_command(
        "ping",
        |ctx, message| async move {
            message.channel_id.say(ctx.http.as_ref(), "Pong!").await;
        }
    )
}