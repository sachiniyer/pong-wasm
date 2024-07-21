use crate::state::State;

use candle_core::{Device, Result, Tensor};
use wasm_bindgen::prelude::*;

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
    w1: Tensor,
    w2: Tensor,
    val: bool,
}

#[wasm_bindgen]
impl Model {
    pub fn new() -> Model {
        let device = Device::Cpu;
        Model {
            val: false,
            w1: Tensor::randn(0f32, 1.0, (784, 100), &device).unwrap_throw(),
            w2: Tensor::randn(0f32, 1.0, (784, 100), &device).unwrap_throw(),
        }
    }

    pub fn direction(&self, state: State) -> i8 {
        ACTION.up
    }

    pub fn update(&mut self, state: State, direction: i8) {
        self.val = !self.val;
    }
}
