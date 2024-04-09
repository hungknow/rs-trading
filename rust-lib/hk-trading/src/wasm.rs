use std::{any::Any, cell::RefCell, rc::Rc};

use hk_dispatcher::{
    ffi_request::HkFFIRequest,
    ffi_response::{HkFFIResponse, HkFFIStatusCode},
    Dispatcher,
};
use lazy_static::lazy_static;
use protobuf::Message;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{
    future_to_promise,
    js_sys::{self, Uint8Array},
};

lazy_static! {
    static ref APP_CORE: RefCellAppCore = RefCellAppCore::new();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn onHkNotify(event_name: &str, payload: JsValue);
}

struct RefCellAppCore(RefCell<Option<AppWASMCore>>);

/// safety:
/// In a WebAssembly, implement the Sync for RefCellAppCore is safety
/// since WASM currently operates in a single-threaded environment.
unsafe impl Sync for RefCellAppCore {}

impl RefCellAppCore {
    fn new() -> Self {
        Self(RefCell::new(None))
    }

    fn dispatcher(&self) -> Option<Rc<Dispatcher>> {
        self.0
            .borrow()
            .as_ref()
            .map(|core| core.event_dispatcher.clone())
    }
}

// #[wasm_bindgen]
// pub fn init_wasm_core() -> js_sys::Promise {
//     let future = async move {
//         info!("🔥🔥🔥Initialized HKWASMCore");
//         Ok(JsValue::from_str(""))
//     };
//     future_to_promise(future)
// }

pub struct AppWASMCore {
    pub event_dispatcher: Rc<Dispatcher>,
}

impl AppWASMCore {
    pub fn new() -> Self {
        let dispatcher = Dispatcher::new();
        Self {
            event_dispatcher: Rc::new(dispatcher),
        }
    }
}

#[wasm_bindgen]
pub fn init_wasm_core() -> js_sys::Promise {
    let future = async move { todo!() };
    future_to_promise(future)
}

#[wasm_bindgen]
pub fn async_send_ffi_request(request_bytes: Vec<u8>) -> js_sys::Promise {
    if let Some(dispatcher) = APP_CORE.dispatcher() {
        // Convert the bytes to a FFIRequest
        let result = HkFFIRequest::parse_from_bytes(&request_bytes);
        match result {
            Ok(request) => {
                // let the dispatcher handle the request
                // async_send_ffi_request_internal(request)
                let future = async move {
                    match dispatcher.dispatch_request(&request).await {
                        Ok(resp) => {
                            let array_buffer_js = resp.write_to_bytes().unwrap();
                            Ok(unsafe {
                                js_sys::Uint8Array::new(&Uint8Array::view(&array_buffer_js)).into()
                            })
                        }
                        Err(err) => {
                            // error!("Error dispatching request: {:?}", err);
                            let mut resp = HkFFIResponse::default();
                            resp.code = HkFFIStatusCode::Err.into();
                            resp.payload = format!("{}", err).into_bytes();
                            let array_buffer_js = resp.write_to_bytes().unwrap();
                            Err(unsafe {
                                js_sys::Uint8Array::new(&Uint8Array::view(&array_buffer_js)).into()
                            })
                        }
                    }
                };
                // Return the result
                future_to_promise(future)
            }
            Err(err) => {
                future_to_promise(async move { Err(JsValue::from_str(&format!("{}", err))) })
            }
        }
    } else {
        future_to_promise(async { Err(JsValue::from_str("Dispatcher is not initialized")) })
    }
}

fn async_event(event_name: &str, payload: Vec<u8>) -> js_sys::Promise {
    let request1 = HkFFIRequest::new();
    let request1_bytes = request1.write_to_bytes().unwrap();

    let request2 = HkFFIRequest::parse_from_bytes(&request1_bytes).unwrap();
    let type_name = HkFFIRequest::type_id(&request1);

    // let request = FFIRequest::from_u8_pointer(input, len).into();

    future_to_promise(async { Err(JsValue::from_str("Dispatcher is not initialized")) })
}

pub fn on_event(event_name: &str, args: JsValue) {
    onHkNotify(event_name, args);
}
