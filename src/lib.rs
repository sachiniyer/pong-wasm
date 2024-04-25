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

let calls = 0;

#[wasm_bindgen]
pub struct State {
    state: Vec<Vec<bool>>,
}

#[wasm_bindgen]
pub struct Model {
    val: bool,
}

#[wasm_bindgen]
impl State {
    pub fn new(data: Vec<f64>, dimension: u16) -> State {
        let mut state = Vec::new();
        for i in 0..dimension {
            let mut row = Vec::new();
            for j in 0..dimension {
                row.push(data[(i * dimension + j) as usize] > 0.5);
            }
            state.push(row);
        }
        State { state }
    }
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
pub fn handle_state(state: State) {
    console::log_1(&"Received a state".into());

    console::log_1(&state.state.len().into());
}
