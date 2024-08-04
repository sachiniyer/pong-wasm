use crate::model::Model;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

pub type Image = Vec<u8>;

#[derive(Copy, Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Distribution {
    up: f64,
    down: f64,
    stay: f64,
}

impl Distribution {
    pub fn new(up: f64, down: f64, stay: f64) -> Distribution {
        Distribution { up, down, stay }
    }
}

#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct State {
    img: Image,
    dist: Distribution,
    choice: u8,
}

impl State {
    pub fn new(img: Image, dist: Distribution, choice: u8) -> State {
        State { img, dist, choice }
    }
}

#[wasm_bindgen]
pub fn new_image(data: Vec<f64>, dimension: u16) -> Image {
    let mut img = Vec::new();
    for i in 0..dimension {
        for j in 0..dimension {
            img.push((data[(i * dimension + j) as usize] * 255.0) as u8);
        }
    }
    img
}

/// A representation of a full game
#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct Sequence {
    /// The sequence of states
    sequence: Vec<State>,
    /// The outcome of the sequence
    outcome: bool,
}

/// A representation for LocalState in browser
#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct LocalState {
    /// The weights of the model
    weights: String,
    /// The history of the games played
    history: Vec<Sequence>,
    /// The current game being played
    current: Vec<State>,
}

/// Utility function to read a model from browser storage
pub fn read_model() -> Model {
    let window = web_sys::window().unwrap();
    if let Some(storage) = window.local_storage().unwrap() {
        if let Some(model) = storage.get_item("model").ok().unwrap() {
            return serde_json::from_str(&model).ok().unwrap();
        }
    }
    return Model::new();
}

/// Utility function to write a model to the browser storage
pub fn write_model(model: Model) {
    let window = web_sys::window().unwrap();
    if let Some(storage) = window.local_storage().unwrap() {
        storage
            .set_item("model", &serde_json::to_string(&model).unwrap())
            .unwrap();
    }
}

/// Utility function to read a state from browser storage
pub fn read_state() -> LocalState {
    let window = web_sys::window().unwrap();
    if let Some(storage) = window.local_storage().unwrap() {
        if let Some(state) = storage.get_item("state").ok().unwrap() {
            return serde_json::from_str(&state).ok().unwrap();
        }
    }
    return LocalState {
        weights: String::new(),
        history: Vec::new(),
        current: Vec::new(),
    };
}

/// Utility function to write an update to the browser storage
pub fn write_state(state: LocalState) {
    let window = web_sys::window().unwrap();
    if let Some(storage) = window.local_storage().unwrap() {
        storage
            .set_item("state", &serde_json::to_string(&state).unwrap())
            .unwrap();
    }
}

/// Adds a frame to the current game in browser storage
pub fn add_frame(frame: State) {
    let mut state = read_state();
    state.current.push(frame);
    write_state(LocalState {
        weights: state.weights.clone(),
        history: state.history.clone(),
        current: state.current.clone(),
    })
}

/// Dumps the current game to the history in browser storage
pub fn dump_game(outcome: bool) -> Sequence {
    let mut state = read_state();
    let new_history = Sequence {
        sequence: state.current.clone(),
        outcome,
    };
    state.history.push(new_history.clone());
    write_state(LocalState {
        weights: state.weights.clone(),
        history: state.history.clone(),
        current: Vec::new(),
    });
    new_history
}
