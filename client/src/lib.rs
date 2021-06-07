use sauron::js_sys::TypeError;
use sauron::prelude::*;
use sauron::web_sys::Response;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
}
