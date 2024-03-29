use std::{any::Any, collections::HashMap, marker::PhantomData, sync::Arc};

/*
- Handler for message
    - Handler can be a function
    - Handler can be a struct that implements a trait
- Register handler for message type
- Returns the ID to unregister the handler later
*/

trait Handle {
    fn call(&self, args: Box<dyn Any>);
}

trait ArgumentCall<A: Clone> {
    fn call(&self, args: A);
}

impl<Func, A> ArgumentCall<A> for Func
where
    Func: Fn(A),
    A: Clone,
{
    fn call(&self, (A): (A)) {
        self(A);
    }
}

struct ArgumentCallHolder<A: Clone + 'static, H: ArgumentCall<A>> {
    handle: H,
    _p: PhantomData<A>,
}

impl<A, H> Handle for ArgumentCallHolder<A, H>
where
    H: ArgumentCall<A>,
    A: Clone,
{
    fn call(&self, args: Box<dyn Any>) {
        if let Some(real_arg) = args.downcast_ref::<A>() {
            self.handle.call(real_arg.clone());
        }
    }
}

pub struct EventEmitter {
    listeners: HashMap<&'static str, Vec<Box<dyn Handle>>>,
}

impl EventEmitter {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    fn on(&mut self, event: &'static str, listener: impl Handle + 'static) {
        self.listeners
            .entry(event)
            .or_insert(Vec::new())
            .push(Box::new(listener));
    }

    fn emit<T: Clone + 'static>(&self, event: &'static str, data: T) {
        self.listeners
            .get(event)
            .map(|listeners| {
                listeners.iter().for_each(|listener| {
                    listener.call(Box::new(data.clone()));
                });
            });
    }
}

pub struct EventHandler<T: 'static> {
    handler: Arc<Box<dyn Fn(T)>>,
}
impl<T> EventHandler<T> {
    pub fn new(handler: Box<dyn Fn(T)>) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

impl<A: Clone> Handle for EventHandler<A> {
    fn call(&self, args: Box<dyn Any>) {
        // println!("EventHandler called with value: {:?}", args);
        if let Some(real_arg) = args.downcast_ref::<A>() {
            (self.handler)(real_arg.clone());
        } else {
            println!("EventHandler called with wrong type");
        }
    }
}

fn event_handler_create<T>(handler: impl Fn(T) + 'static) -> EventHandler<T> {
    EventHandler::new(Box::new(handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_emitter() {
        let mut emitter = EventEmitter::new();
        let event_i32 = "i32_event";
        let event_u32 = "u32_event";
        let uevent_multi_arg = "multi_arg_event";

        // Register the event listener by handler
        let handler = event_handler_create(|val: i32| {
            println!("Handler called with value: {}", val);
        });
        emitter.on(event_i32, handler);

        // Emit the event
        let data = "hello";
        let data_i32  = 10;
        emitter.emit(event_i32, "hello");
        emitter.emit(event_i32, data);
        emitter.emit(event_i32, data_i32);

        let u32_handler = event_handler_create(|val: u32| {
            println!("Handler event_u32 called with value: {}", val);
        });
        emitter.on(event_u32, u32_handler);
        emitter.emit(event_u32, 100u32);

        let multi_argument_handler = event_handler_create(|val: (i32, u32)| {
            println!("Handler uevent_multi_arg called with value: {:?}", val);
        });
        emitter.on(uevent_multi_arg, multi_argument_handler);
        emitter.emit(uevent_multi_arg, (10i32, 100u32));
    }
}
