use crate::model::Model;
use crate::consts::{ MODEL_DB, MODEL_DB_KEY, MODEL_DB_KEY_VERSION };

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
use rexie::*;

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
    outcome: Option<bool>,
}

/// A representation for LocalState in browser
#[derive(Clone, Deserialize, Serialize)]
#[wasm_bindgen]
pub struct LocalState {
    /// The history of the games played
    history: Vec<Sequence>,
    /// The current game being played
    current: Vec<State>,
}

/// Utility function to read a model from browser storage
pub async fn read_model() -> Result<Model> {
    let rexie = Rexie::builder(MODEL_DB)
        .build()
        .await?;
    if rexie.store_names().contains(&MODEL_DB.to_string()) {
        let transaction = rexie.transaction(&[MODEL_DB], TransactionMode::ReadOnly)?;
        let store = transaction.store(MODEL_DB)?;
        let model = store.get(MODEL_DB_KEY_VERSION.into()).await?.unwrap();
        let model: Model = serde_wasm_bindgen::from_value(model).unwrap();
        return Ok(model);
    }
    let rexie = Rexie::builder(MODEL_DB)
       .version(1)
       .add_object_store(
           ObjectStore::new(MODEL_DB)
               .key_path(MODEL_DB_KEY)
       )
       .build()
       .await?;
    let transaction = rexie.transaction(&[MODEL_DB], TransactionMode::ReadWrite)?;
    let store = transaction.store(MODEL_DB)?;
    let model = Model::new();
    let model_js = serde_wasm_bindgen::to_value(&model).unwrap();
    let key = Some(JsValue::from_f64(MODEL_DB_KEY_VERSION));
    store.add(&model_js, key.as_ref()).await?;
    return Ok(model);
}

// /// Utility function to write a model to the browser storage
// pub fn write_model(model: Model) {}

// /// Utility function to read a state from browser storage
// pub fn read_state() -> LocalState {}

// /// Utility function to write an update to the browser storage
// pub fn write_state(state: LocalState) {}

// /// Adds a frame to the current game in browser storage
// pub fn add_frame(frame: State) {}

// /// Dumps the current game to the history in browser storage
// pub fn end_game(outcome: bool) -> Sequence {}
