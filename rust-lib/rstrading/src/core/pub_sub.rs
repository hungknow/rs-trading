use std::collections::HashMap;
use std::hash::Hash;

pub trait Identifier {
    fn id(&self) -> &str;
}

pub struct SyncPublisher<EventId, EventType>
where
    EventId: Hash + Eq,
    EventType: Clone + Hash,
{
    events: HashMap<EventId, Vec<Box<dyn Fn(EventType)>>>,
}

impl<EventId, EventType> SyncPublisher<EventId, EventType>
where
    EventId: Hash + Eq,
    EventType: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }
}

impl<EventId, EventType> SyncPublisher<EventId, EventType>
where
    EventId: Hash + Eq,
    EventType: Clone + Hash,
{
    pub fn subscribe(&mut self, event_type: EventId, listener: Box<dyn Fn(EventType)>) {
        let a = self.events.entry(event_type).or_insert(vec![]);
        a.push(listener);
    }

    // pub fn unsubscribe(&mut self, event_type: EventId, listener: Box<dyn Fn(EventType)>) {
    //     self.events
    //         .get_mut(&event_type)
    //         .unwrap()
    //         .retain(|&x| x != listener);
    // }

    pub fn notify(&self, event_type: EventId, message: EventType) {
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
        let mut publisher = SyncPublisher::<&str, &str>::new();
        let listener = Box::new(|message: &str| assert_eq!(message, "hello"));
        publisher.subscribe("test", listener);
        publisher.notify("test", "hello");
    }
}
