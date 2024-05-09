use candle_core::{Device, Result, Tensor};
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

#[derive(Serialize, Deserialize)]
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
    let _worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within Wasm".into());
}

#[wasm_bindgen]
pub fn handle_state(state: State) {
    console::log_1(&serde_json::to_value(state).unwrap().to_string().into());
    // store the tuples into local state (states, actions, rewards)
}

#[wasm_bindgen]
pub fn handle_end() {
    // use the local state tuples to train the model
    // checkpoint the model into local state as well
}
