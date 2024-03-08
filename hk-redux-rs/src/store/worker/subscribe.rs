use crate::store::worker::Work;
use crate::subscriber::SubscriberAsync;
use crate::Subscriber;
use std::marker::PhantomData;

pub struct Subscribe<State> {
    subscriber: Box<dyn Subscriber<State> + Send>,
    _types: PhantomData<State>,
}

impl<State> Subscribe<State> {
    pub fn new(subscriber: Box<dyn Subscriber<State> + Send>) -> Self {
        Subscribe {
            subscriber,
            _types: Default::default(),
        }
    }

    pub fn into_subscriber(self) -> Box<dyn Subscriber<State> + Send> {
        self.subscriber
    }
}

impl<State> Work for Subscribe<State>
where
    State: Send,
{
    type Result = ();
}

// pub struct SubscribeAsync<State> {
//     subscriber: Box<dyn SubscriberAsync<State> + Send>,
//     _types: PhantomData<State>,
// }

// impl<State> SubscribeAsync<State> {
//     pub fn new(subscriber: Box<dyn SubscriberAsync<State> + Send>) -> Self {
//         SubscribeAsync {
//             subscriber,
//             _types: Default::default(),
//         }
//     }

//     pub fn into_subscriber(self) -> Box<dyn SubscriberAsync<State> + Send> {
//         self.subscriber
//     }
// }

// impl<State> Work for SubscribeAsync<State>
// where
//     State: Send,
// {
//     type Result = ();
// }
