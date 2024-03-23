use std::any::Any;

use protobuf::Message;
use tracing::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{future_to_promise, js_sys};

use crate::protos::ffi_request::FFIRequest;

// #[wasm_bindgen]
// pub fn init_wasm_core() -> js_sys::Promise {
//     let future = async move {
//         info!("🔥🔥🔥Initialized HKWASMCore");
//         Ok(JsValue::from_str(""))
//     };
//     future_to_promise(future)
// }

#[wasm_bindgen]
pub fn async_send_ffi_request(request_bytes: Vec<u8>) -> js_sys::Promise {
    let result = FFIRequest::parse_from_bytes(&request_bytes);
    match result {
        Ok(request) => {
            // async_send_ffi_request_internal(request)
            future_to_promise(async { Ok(JsValue::null()) })
        }
        Err(err) => future_to_promise(async move { Err(JsValue::from_str(&format!("{}", err))) }),
    }
}

fn async_event(event_name: &str, payload: Vec<u8>) -> js_sys::Promise {
    let request1 = FFIRequest::new();
    let request1_bytes = request1.write_to_bytes().unwrap();

    let request2 = FFIRequest::parse_from_bytes(&request1_bytes).unwrap();
    let type_name = FFIRequest::type_id(&request1);

    // let request = FFIRequest::from_u8_pointer(input, len).into();

    future_to_promise(async { Err(JsValue::from_str("Dispatcher is not initialized")) })
}
