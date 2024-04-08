use hk_dispatcher::{ffi_event_type::HkFFIEventType, ffi_request::HkFFIRequest, MessageHandler};
use hk_infra::{future::HkFutureResult, HkError};

pub struct TradingMessageHandler {
    pub message: String,
}

impl MessageHandler<HkFFIRequest, HkFFIRequest> for TradingMessageHandler {
    fn handle_message(&self, message: &HkFFIRequest) -> HkFutureResult<HkFFIRequest, HkError> {
        match message.event.unwrap() {
            HkFFIEventType::HK_FFI_REQ_UNKNOWN => todo!("HK_FFI_REQ_UNKNOWN"),

            HkFFIEventType::HK_FFI_REQ_SYMBOL_GET_INFO => {
                todo!("HK_FFI_REQ_SYMBOL_GET_INFO");
            }
            HkFFIEventType::HK_FFI_REQ_CHART_GET_SVG => todo!(),
            HkFFIEventType::HK_FFI_RES_CHART_GET_SVG => todo!(),
        }
    }
}
