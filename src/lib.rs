use candle_core::{Device, Result, Tensor};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, Worker};

struct Action {
    up: i8,
    down: i8,
    stay: i8,
}

const ACTION: Action = Action {
    up: 1,
    down: -1,
    stay: 0,
};

#[wasm_bindgen]
pub struct Model {
    val: bool,
}

#[wasm_bindgen]
impl Model {
    pub fn new() -> Model {
        Model { val: false }
    }

    pub fn direction(&self, state: State) -> i8 {
        ACTION.up
    }

    pub fn update(&mut self, state: State, direction: i8) {
        self.val = !self.val;
    }
}

#[wasm_bindgen]
pub struct State {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl State {
    pub fn new(x: f64, y: f64) -> State {
        State { x, y }
    }
}

#[wasm_bindgen]
pub fn startup() {
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within Wasm".into());

    let callback = Closure::wrap(Box::new(move |event: web_sys::MessageEvent| {
        console::log_1(&"Received a message from the worker".into());
        console::log_1(&event.data());
    }) as Box<dyn FnMut(_)>);

    worker_handle
        .borrow()
        .add_event_listener_with_callback("message", callback.as_ref().unchecked_ref())
        .unwrap();

    std::mem::forget(callback);
}

#[wasm_bindgen]
pub fn handle_state(state: String) {
    console::log_1(&"Received a state".into());
    console::log_1(&state.len().into());
}
