use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::{Hash, Hasher},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use async_trait::async_trait;
use futures::{stream::FuturesUnordered, StreamExt};
use parking_lot::Mutex;

type EventsMap = HashMap<TypeId, HashMap<ListenerId, Box<dyn Any + Send + Sync>>>;

/***
 * Events
 */
pub trait Event: Any + Send + Sync {
    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

#[async_trait]
pub trait EventHandler<E: Event + Send + Sync + 'static>: Send + Sync {
    async fn handle_event(&mut self, event: &E);
}

/**
 * Listener
 */
#[derive(Clone, Debug, Eq)]
pub struct ListenerId {
    pub id: usize,
    pub type_id: TypeId,
}

impl PartialEq for ListenerId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.type_id == other.type_id
    }
}

impl Hash for ListenerId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.type_id.hash(state);
    }
}

#[derive(Clone)]
pub struct Listener<E: Event> {
    pub(crate) id: ListenerId,
    handler: Arc<Mutex<dyn EventHandler<E>>>,
    pub(crate) once: bool,
}

impl<E: Event> std::fmt::Debug for Listener<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Listener")
            .field("id", &self.id)
            .field("once", &self.once)
            .finish()
    }
}

impl<E: Event + Send + Sync + 'static> Listener<E> {
    pub(crate) fn new(
        handler: impl EventHandler<E> + 'static,
        once: bool,
        listener_id: ListenerId,
    ) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
            once,
            id: listener_id,
        }
    }

    pub(crate) async fn call(&self, event: &E) {
        let mut handler = self.handler.lock();
        handler.handle_event(event).await;
    }
}

/**
 * EventEmitter
 */

pub struct EventEmitter {
    events: EventsMap,
    next_id: AtomicUsize,
}

impl Default for EventEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl EventEmitter {
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
            next_id: AtomicUsize::new(0),
        }
    }

    #[inline]
    pub fn on<E: Event + 'static>(
        &mut self,
        handler: impl EventHandler<E> + 'static,
    ) -> ListenerId {
        self.add_listener(handler, false)
    }

    #[inline]
    pub fn off<E: Event + 'static>(&mut self, listener_id: ListenerId) where E: Clone + Send + 'static {
        self.remove_listener(listener_id)
    }

    pub async fn emit<E: Event + 'static>(&self, event: E)
    where
        E: Clone + Send + 'static, // Ensure E is Clone and Send
    {
        let listeners = {
            self.events
                .get(&TypeId::of::<E>())
                .map(|listeners_for_type| {
                    listeners_for_type
                        .iter()
                        .filter_map(|(_, listener)| listener.downcast_ref::<Listener<E>>())
                        .cloned() // Clone each listener
                        .collect::<Vec<_>>() // Collect into a Vec
                })
        };

        let unordered_fut: FuturesUnordered<_> = FuturesUnordered::new();

        if let Some(ref listeners) = listeners {
            for listener in listeners.clone() {
                let event_clone = event.clone(); // Clone event for each listener
                let item = async move { listener.call(&event_clone).await };
                unordered_fut.push(item);
            }
        }

        unordered_fut.for_each(|v| futures::future::ready(())).await;
    }

    fn add_listener<E: Event + 'static>(
        &mut self,
        handler: impl EventHandler<E> + 'static,
        once: bool,
    ) -> ListenerId {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let type_id = TypeId::of::<E>();
        let listener_id = ListenerId { id, type_id };
        let listener = Listener::new(handler, once, listener_id.clone());

        self.events
            .entry(type_id)
            .or_default()
            .insert(listener_id.clone(), Box::new(listener));

        listener_id
    }

    pub fn remove_listener(&mut self, listener_id: ListenerId) {
        if let Some(mut listeners_for_type) = self.events.get_mut(&listener_id.type_id) {
            listeners_for_type.remove(&listener_id);
        }
    }

    pub fn event_listeners_count<E: Event + 'static>(&self) -> Option<usize>
    where
        E: Clone + Send + 'static, // Ensure E is Clone and Send
    {
        self.events.get(&TypeId::of::<E>()).map(|listeners| listeners.len())
    }

    pub fn get_event_listeners<E: Event + 'static>(&self) -> Option<Vec<Listener<E>>> where E: Clone + Send + 'static {
        let event_listeners = {
            self.events.get(&TypeId::of::<E>()).map(|listeners_of_type| {
                listeners_of_type
                    .iter()
                    .filter_map(|(_, listener)| listener.downcast_ref::<Listener<E>>())
                    .cloned() // Clone each listener
                    .collect::<Vec<_>>() // Collect into a Vec
            })
        };
        event_listeners
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[derive(Clone)]
    struct TestEvent {
        id: usize,
    }

    impl Event for TestEvent {}

    struct TestEventHandler {
        called: AtomicUsize,
    }

    #[async_trait]
    impl EventHandler<TestEvent> for TestEventHandler {
        async fn handle_event(&mut self, _event: &TestEvent) {
            println!("TestEvent handled");
            self.called.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn test_event_emitter() {
        futures::executor::block_on(async {
            let mut emitter = EventEmitter::new();
            let handler = TestEventHandler {
                called: AtomicUsize::new(0),
            };

            let listener_id = emitter.on::<TestEvent>(handler);

            let event = TestEvent { id: 1 };
            emitter.emit(event).await;

            // assert_eq!(handler.called.load(Ordering::Relaxed), 1);
        });
    }

    #[derive(Clone)]
    enum EventType1 {
        Event1,
        Event2,
    }

    impl Event for EventType1 {}

    #[async_trait]
    impl EventHandler<EventType1> for TestEventHandler {
        async fn handle_event(&mut self, event: &EventType1) {
            println!("EventType1 handled");
            self.called.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn test_enum() {
        futures::executor::block_on(async {
            let mut emitter = EventEmitter::new();
            let handler = TestEventHandler {
                called: AtomicUsize::new(0),
            };

            let listener_id = emitter.on::<EventType1>(handler);

            let event1 = EventType1::Event1;
            emitter.emit(event1).await;

            let event2= EventType1::Event1;
            emitter.emit(event2).await;

            
            emitter.off::<EventType1>(listener_id);

            let event3 = EventType1::Event1;
            emitter.emit(event3).await;
        });
    }
}
