use hk_infra::{future::HkFutureResult, HkError};

use crate::protos::{ffi_request::HkFFIRequest, ffi_response::HkFFIResponse};

/*
The struct implement the trait MessageHandler
*/
pub trait MessageHandler<R, S> {
    fn handle_message(&self, message: &R) -> HkFutureResult<S, HkError>;
}

#[cfg(test)]
mod tests {
    use protobuf::Message;

    use super::*;
    use crate::protos::add_event::{AddEventRequest, AddEventResponse};

    struct TestMessageHandler {}

    impl MessageHandler<AddEventRequest, AddEventResponse> for TestMessageHandler {
        fn handle_message(
            &self,
            message: &AddEventRequest,
        ) -> HkFutureResult<AddEventResponse, HkError> {
            let a = message.a;
            let b = message.b;
            HkFutureResult::new(async move {
                let mut response = AddEventResponse::default();
                response.result = a + b;
                Ok((response))
            })
        }
    }

    #[tokio::test]
    async fn test_message_handler() {
        let mut handler = TestMessageHandler {};
        let mut add_event = AddEventRequest::new();
        add_event.a = 10;
        add_event.b = 20;
        let result = handler.handle_message(&add_event).await;
        match result {
            Ok(response) => {
                assert_eq!(response.result, 30);
            }
            Err(e) => {
                panic!("Error: {:?}", e);
            }
        }
    }
}
