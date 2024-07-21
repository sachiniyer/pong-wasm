pub mod model;
pub mod state;

use crate::state::{add_frame, dump_game, State};

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, Worker};

/*
 * PLAN:
 * - Use handle_state to take data for RL model
 * - Store weights in browser Storage (localStorage)
 * - On inference call, take state and publish results.
 */

#[wasm_bindgen]
pub fn startup() {
    let _worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    #[allow(unsafe_code)]
    unsafe {
        console::log_1(&"Created a new worker from within Wasm".into());
    }
}

#[wasm_bindgen]
pub fn handle_state(new_state: State) {
    add_frame(new_state);
}

#[wasm_bindgen]
pub fn handle_end(outcome: bool) {
    dump_game(outcome);
}
