use crate::discord::app::creators::{create_prefix_command, PrefixCommand};

pub fn ping_command() -> PrefixCommand {
    create_prefix_command("!ping".to_string(), |message, client| async move {
        if message.author.bot || message.guild_id.is_none() {
            return;
        }

        let channel_id = message.channel_id;
        let result = client.create_message(channel_id).content("Pong 🏓").await;

        if let Err(err) = result {
            eprintln!("Error trying to responde !Ping command: {:?}", err);
        }
    })
}
