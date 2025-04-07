mod ban_add;
mod member_added;
mod member_removed;
mod message_create;
mod interaction_create;

use super::app::creators::EventCallback;
use std::collections::HashMap;
use twilight_model::gateway::event::EventType;

type EventsMap = HashMap<EventType, EventCallback>;

pub fn app_events() -> HashMap<EventType, EventCallback> {
    let mut events = EventsMap::new();
    // Add more commands here...
    let member_added_event = member_added::member_added();
    events.insert(member_added_event.event, member_added_event.reply);
    let member_removed_event = member_removed::member_removed();
    events.insert(member_removed_event.event, member_removed_event.reply);
    let ban_add = ban_add::ban_add();
    events.insert(ban_add.event, ban_add.reply);
    let message_create = message_create::message_create();
    events.insert(message_create.event, message_create.reply);
    let interaction_create = interaction_create::interaction_create();
    events.insert(interaction_create.event, interaction_create.reply);

    events
}
