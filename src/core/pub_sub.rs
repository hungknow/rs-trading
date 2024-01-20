use std::collections::HashMap;
use std::hash::Hash;

pub struct SyncPublisher<EventType, Subscriber>
where
    EventType: Clone + Eq + Hash,
    Subscriber: Fn(EventType),
{
    events: HashMap<EventType, Vec<Subscriber>>,
}

impl<EventType, Subscriber> SyncPublisher<EventType, Subscriber>
where
    EventType: Clone + Eq + Hash,
    Subscriber: Fn(EventType),
{
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }
}

impl<EventType, Subscriber> SyncPublisher<EventType, Subscriber>
where
    EventType: Clone + Eq + Hash,
    Subscriber: Fn(EventType) + PartialEq + Copy,
    // Subscriber: Copy + PartialEq,
    // Data: Clone,
{
    fn subscribe(&mut self, event_type: EventType, listener: Subscriber) {
        self.events.entry(event_type.clone()).or_default();
        self.events.get_mut(&event_type).unwrap().push(listener);
    }

    fn unsubscribe(&mut self, event_type: EventType, listener: Subscriber) {
        self.events
            .get_mut(&event_type)
            .unwrap()
            .retain(|&x| x != listener);
    }

    fn notify(&self, event_type: EventType, message: EventType) {
        let listeners = self.events.get(&event_type).unwrap();
        for listener in listeners {
            listener(message.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_publisher() {
        let mut publisher = SyncPublisher::<&str, fn(&str)>::new();
        let listener = |message: &str| {
            assert_eq!(message, "hello")
        };
        publisher.subscribe("test", listener);
        publisher.notify("test", "hello");
    }
}