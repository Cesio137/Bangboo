use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn run(message: Box<MessageCreate>, client: Arc<Client>) {
    if message.author.bot || message.guild_id.is_none() {
        return;
    }

    let channel_id = message.channel_id;
    let result = client.create_message(channel_id).content("Pong 🏓").await;

    if let Err(err) = result {
        eprintln!("Error trying to responde !Ping command: {:?}", err);
    }
}
