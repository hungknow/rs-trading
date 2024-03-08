pub trait EventListener<EventType> {
    fn event_listener_add(&mut self, event_name: EventType, event: event_listener::Event);
}
