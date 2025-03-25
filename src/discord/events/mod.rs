mod member_added;

use std::collections::HashMap;
use twilight_model::gateway::event::EventType;
use super::app::creators::EventCallback;

type EventsMap = HashMap<EventType, EventCallback>;

pub fn app_events() -> HashMap<EventType, EventCallback> {
    let mut events = EventsMap::new();
    // Add more commands here...
    let member_added_event = member_added::member_added();
    events.insert(member_added_event.event, member_added_event.reply);

    events
}