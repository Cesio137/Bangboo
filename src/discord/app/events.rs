use super::creators::EventCallback;
use crate::discord::events::app_events;
use std::collections::HashMap;
use twilight_model::gateway::event::EventType;

pub struct AppEvents {
    pub events: HashMap<EventType, EventCallback>,
}

impl AppEvents {
    pub fn new() -> Self {
        let events = app_events();
        AppEvents { events: events }
    }
}
