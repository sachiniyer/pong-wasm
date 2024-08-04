pub mod consts;
pub mod model;
pub mod state;

use crate::state::{add_frame, dump_game, read_model, write_model, Image, State};

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
    console::log_1(&"Created a new worker from within Wasm".into());
}

#[wasm_bindgen]
pub fn handle_state(img: Image) -> u8 {
    // infer with weights that you take from local storage here.
    let model = read_model();
    let inference = model.infer(img.clone());
    add_frame(State::new(img, inference.dist, inference.choice));
    return inference.choice;
}

#[wasm_bindgen]
pub fn handle_end(outcome: bool) {
    let model = read_model();
    model.train(dump_game(outcome));
    write_model(model);
}
