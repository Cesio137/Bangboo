use crate::discord::app::creators::{create_prefix_command, PrefixCommand};

pub fn ping_command() -> PrefixCommand {
    create_prefix_command(
        "!ping".to_string(),
        |message, client| async move {
            let channel_id = message.channel_id;
            let result = client.create_message(channel_id)
                .content("Pong 🏓")
                .await;

            if let Err(why) = result {
                eprintln!("Error trying to responde !Ping command: {:?}", why);
            }
        }
    )
}