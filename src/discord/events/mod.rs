mod ban_added;
mod interaction_create;
mod member_added;
mod member_removed;
mod message_create;
mod ready;

use super::app::base::App;
use colored::Colorize;
use serenity::all::{async_trait, Context, Event, EventHandler, FullEvent, GuildId, Interaction, Member, Message, RatelimitInfo, Ready, User};
use serenity::model::application;

#[async_trait]
impl EventHandler for App {
    async fn dispatch(&self, _context: &Context, _event: &FullEvent) {
        match _event {
            FullEvent::Ready { data_about_bot, .. } =>
                ready::run(self, _context, data_about_bot).await,
            
            FullEvent::GuildBanAddition { guild_id, banned_user, .. } => 
                ban_added::run(self, _context, guild_id, banned_user).await,
            
            FullEvent::InteractionCreate { interaction, .. } =>
                interaction_create::run(self, _context, interaction).await,
            
            FullEvent::GuildMemberAddition { new_member, ..} =>
                member_added::run(self, _context, new_member).await,
            
            FullEvent::GuildMemberRemoval { guild_id, user, member_data_if_available, .. } =>
                member_removed::run(self, _context, guild_id, user).await,
            
            FullEvent::Message { new_message, .. } =>
                message_create::run(self, _context, new_message).await,
            
            _ => {}
        }
    }
}
