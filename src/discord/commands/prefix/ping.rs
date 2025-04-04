use crate::discord::app::creators::{create_prefix_command, PrefixCommand};

pub fn ping_command() -> PrefixCommand {
    create_prefix_command("!ping".to_string(), |context| async move {
        if context.message.author.bot || context.message.guild_id.is_none() {
            return;
        }

        let channel_id = context.message.channel_id;
        let result = context.client.create_message(channel_id).content("Pong 🏓").await;

        if let Err(err) = result {
            eprintln!("Error trying to responde !Ping command: {:?}", err);
        }
    })
}
