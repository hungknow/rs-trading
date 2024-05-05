use std::{collections::HashMap, hash::Hash};
use futures::{stream::FuturesUnordered, StreamExt};
use crate::future::{HkBoxFuture, HkConcurrent};
use parking_lot::Mutex;

#[derive(Debug)]
pub enum AsyncDispatchResult {
    /// Stops the listener from receiving further events from the dispatcher.
    StopListening,
}

pub trait AsyncListener<T>
where T: PartialEq + Eq + Hash + Clone + HkConcurrent + 'static
{
    fn on_event(&mut self, event: &T) -> HkBoxFuture<Option<AsyncDispatchResult>>;
}

pub struct AsyncDispatcher<T>
where T: PartialEq + Eq + Hash + Clone + HkConcurrent + 'static
{
    events: HashMap<T, Vec<Box<dyn AsyncListener<T> + 'static>>>,
}

impl<T> AsyncDispatcher<T>
where
    T: PartialEq + Eq + Hash + Clone + HkConcurrent + Sized + 'static,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            events: HashMap::new(),
        }
    }

    pub fn add_listener<D: AsyncListener<T> + 'static>(&mut self, event_key:T, listener: D) {
        let listener = Box::new(listener);
        self.events
            .entry(event_key)
            .or_insert(Vec::new())
            .push(listener);
    }

    pub async fn dispatch_event<'a>(&mut self, event_identifier: &T) {
        if let Some(listeners) = self.events.get_mut(event_identifier) {
            let unordered_fut: FuturesUnordered<_> = FuturesUnordered::new();
            for (id, listener) in listeners.iter_mut().enumerate() {
                let item = async move {(id, listener.on_event(event_identifier).await)};
                unordered_fut.push(item);
            }

            let listeners_to_remove = Mutex::new(Vec::<usize>::new());

            unordered_fut.for_each(|v| {
                if let Some(AsyncDispatchResult::StopListening) = v.1 {
                    listeners_to_remove.lock().push(v.0);
                }

                futures::future::ready(())
            }).await;

            listeners_to_remove.lock().iter().for_each(|id| {
                listeners.swap_remove(*id);
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestListener {
        id: usize,
        called: usize,
    }

    impl TestListener {
        fn new(id: usize) -> Self {
            Self {
                id,
                called: 0,
            }
        }
    }

    impl AsyncListener<usize> for TestListener {
        fn on_event(&mut self, event: &usize) -> HkBoxFuture<Option<AsyncDispatchResult>> {
            self.called += 1;
            let id: usize = self.id;
            let event_val = *event;
            Box::pin(async move {
                println!("Listener {} received event: {}", id, event_val);
                // HkBoxFuture::new(async move {None})
                None
            })
        }
    }

    // #[tokio::test]
    #[test]
    fn test_async_dispatcher() {
        futures::executor::block_on(async {
            let mut dispatcher = AsyncDispatcher::new();
            let listener1 = TestListener::new(1);
            let listener2 = TestListener::new(2);
            let listener3 = TestListener::new(3);
            let listener4 = TestListener::new(4);

    
            dispatcher.add_listener(1, listener1);
            dispatcher.add_listener(1, listener2);
            dispatcher.add_listener(1, listener3);
            dispatcher.add_listener(2, listener4);

    
            dispatcher.dispatch_event(&1).await;
        });
    }
}