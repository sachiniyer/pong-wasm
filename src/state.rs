use crate::State;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

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
    let serialized = web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .get_item("state")
        .unwrap()
        .unwrap();
    serde_json::from_str(&serialized).unwrap()
}

/// Utility function to write an update to the browser storage
fn write_state(state: LocalState) {
    let serialized = serde_json::to_string(&state).unwrap();
    web_sys::window()
        .unwrap()
        .local_storage()
        .unwrap()
        .unwrap()
        .set_item("state", &serialized)
        .unwrap();
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
