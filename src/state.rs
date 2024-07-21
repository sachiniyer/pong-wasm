use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Storage;

#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct State {
    state: Vec<Vec<bool>>,
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

/// A representation of a full game
#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
struct Sequence {
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

/// Utility function to read a state from browser storage
fn read_state() -> LocalState {
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
fn write_state(state: LocalState) {
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
pub fn dump_game(outcome: bool) {
    let mut state = read_state();
    let new_history = Sequence {
        sequence: state.current.clone(),
        outcome,
    };
    state.history.push(new_history);
    write_state(LocalState {
        weights: state.weights.clone(),
        history: state.history.clone(),
        current: Vec::new(),
    });
}
