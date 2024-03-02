use candle_core::{Device, Result, Tensor};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{console, HtmlElement, HtmlInputElement, MessageEvent, Worker};

#[wasm_bindgen]
pub struct Model {
    val: bool,
}

#[wasm_bindgen]
pub struct State {}

#[wasm_bindgen]
pub enum Direction {
    Up,
    Down,
}

#[wasm_bindgen]
impl Model {
    pub fn new() -> Model {
        Model { val: false }
    }

    pub fn direction(&self, state: State) -> bool {
        self.val
    }

    pub fn update(&mut self, state: State, direction: Direction) {
        self.val = !self.val;
    }
}

/// Run entry point for the main thread.
#[wasm_bindgen]
pub fn startup() {
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    console::log_1(&"Created a new worker from within Wasm".into());
}
