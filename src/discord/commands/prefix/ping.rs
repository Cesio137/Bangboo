use crate::discord::app::creators::{create_prefix_command, PrefixCommand};

pub fn ping_command() -> PrefixCommand {
    create_prefix_command(
        "ping", 
        |message, client| async move {
            if message.author.bot || message.guild_id.is_none() { return Ok(()); }

            let channel_id = message.channel_id;
            client.create_message(channel_id).content("Pong 🏓").await?;
            Ok(())
        }
    )
}