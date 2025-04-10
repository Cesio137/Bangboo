use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::gateway::payload::incoming::MessageCreate;

pub async fn run(message: Box<MessageCreate>, client: Arc<Client>) -> Result<()> {
    if message.author.bot || message.guild_id.is_none() { return Ok(()); }

    let channel_id = message.channel_id;
    client.create_message(channel_id).content("Pong 🏓").await?;
    Ok(())
}
