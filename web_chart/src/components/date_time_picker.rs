// https://docs.rs/leptos_element_plus/latest/src/leptos_element_plus/components/el_date_time_picker.rs.html

use js_sys::Date;
//use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsValue, closure::Closure};
use js_sys::{Array/*,Object,Reflect*/};
//use serde::{Serialize,Deserialize,de::DeserializeOwned};
//use serde_wasm_bindgen::from_value;
pub use super::common::Size;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type{
    Year,
    Month,
    Date,
    #[default]
    DateTime,
    Week,
}


pub fn DateTimePicker() {
    
    view! {

    }
}