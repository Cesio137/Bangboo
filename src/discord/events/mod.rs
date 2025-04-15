mod ban_add;
mod member_added;
mod member_removed;
mod message_create;
mod interaction_create;

use super::app::context::AppContext;
use anyhow::Result;
use std::sync::Arc;
use twilight_gateway::Event;

pub async fn app_events(event: Event, context: Arc<AppContext>) -> Result<()> {
    match event {
        Event::BanAdd(banadd) => ban_add::run(banadd, context).await?,
        Event::InteractionCreate(interaction) => interaction_create::run(interaction, context).await?,
        Event::MemberAdd(member) => member_added::run(member, context).await?,
        Event::MemberRemove(member) => member_removed::run(member, context).await?,
        Event::MessageCreate(message) => message_create::event(message, context).await?,
        _ => {}
    }
    
    Ok(())
}
