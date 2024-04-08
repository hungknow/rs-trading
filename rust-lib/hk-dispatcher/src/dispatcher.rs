use std::{pin::Pin, sync::Arc};

use hk_infra::{
    future::{HkBoxFuture, HkConcurrent, HkFutureResult},
    HkError,
};

use crate::{
    ffi_response::HkFFIResponse, protos::ffi_request::HkFFIRequest, HkDispatcherRuntime,
    MessageHandler,
};

pub struct FFIRequestHandlerSlot {
    id: usize,
    handler: Pin<Box<dyn MessageHandler<HkFFIRequest, HkFFIResponse>>>,
}

// From the event name, dispatch to all registered callbacks
pub struct Dispatcher {
    // runtime: Arc<HkDispatcherRuntime>,
    // List of message handlers
    handlers: Vec<FFIRequestHandlerSlot>,
}

impl Dispatcher {
    pub fn boxed_async_send_with_callback<Req, Callback>(&self, req: Req, callback: Callback)
    where
        Req: Into<HkFFIRequest> + 'static,
        Callback: FnOnce() -> HkBoxFuture<'static, ()> + HkConcurrent + 'static,
    {
        let request = req.into();
    }

    pub fn add_handler(
        &mut self,
        handler: Pin<Box<dyn MessageHandler<HkFFIRequest, HkFFIResponse>>>,
    ) -> usize {
        //TODO: Handler the concurrent addition
        let current_id = self.handlers.len();
        let slot = FFIRequestHandlerSlot {
            id: current_id,
            handler: handler,
        };
        self.handlers.push(slot);
        current_id
    }

    pub fn remove_handler(&mut self, id: usize) {
        self.handlers.remove(id);
    }

    pub async fn dispatch_request(&self, request: &HkFFIRequest) -> Result<HkFFIResponse, HkError> {
        let handlers = self.handlers.as_slice();

        // Let the handler handle the request one by one until one of them returns a response
        for handler_slot in handlers {
            match handler_slot.handler.handle_message(request).await {
                Ok(response) => return Ok(response),
                Err(err) => match err {
                    HkError::HkFFIUnimplemented(_) => {
                        continue;
                    }
                    _ => {
                        return Err(err);
                    }
                },
            }
        }
        Err(HkError::HkFFIUnimplemented(request.event.value()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestMessageHandler {}

    impl MessageHandler<HkFFIRequest, HkFFIResponse> for TestMessageHandler {
        fn handle_message(&self, message: &HkFFIRequest) -> HkFutureResult<HkFFIResponse, HkError> {
            HkFutureResult::new(async move {
                let mut response = HkFFIResponse::new();
                // response.payload = message.event;
                Ok(response)
            })
        }
    }

    #[tokio::test]
    async fn test_add_handler() {
        let mut dispatcher = Dispatcher {
            // runtime: Arc::new(HkDispatcherRuntime),
            handlers: Vec::new(),
        };

        let handler = TestMessageHandler {};
        let handler_box =
            Box::pin(handler) as Pin<Box<dyn MessageHandler<HkFFIRequest, HkFFIResponse>>>;

        let handler_id = dispatcher.add_handler(handler_box);

        assert_eq!(dispatcher.handlers.len(), 1);
        assert_eq!(dispatcher.handlers[0].id, handler_id);

        let request = HkFFIRequest::new();
        match dispatcher.dispatch_request(&request).await {
            Ok(_) => {},
            Err(err) => assert!(matches!(err, HkError::HkFFIUnimplemented(_))),
        }
    }
}
