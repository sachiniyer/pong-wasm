use crate::{
    consts::{DB_NAME, MODEL_DB_KEY, MODEL_DB_KEY_VERSION, MODEL_STORE, STATE_DB_KEY, STATE_STORE},
    model::{Inference, Model},
};

use serde::{Deserialize, Serialize};

use rexie::*;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Math::random;

pub type Image = Vec<u8>;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Distribution {
    up: f32,
    down: f32,
    stay: f32,
}

impl Distribution {
    pub fn new(up: f32, down: f32, stay: f32) -> Distribution {
        Distribution { up, down, stay }
    }

    pub fn sample(&self) -> u8 {
        let rand = random() as f32;
        match rand {
            x if x < self.up => 0,
            x if x < self.up + self.down => 1,
            _ => 2,
        }
    }

    pub fn choice(&self) -> u8 {
        let max = self.up.max(self.down).max(self.stay);
        match max {
            x if x == self.up => 0,
            x if x == self.down => 1,
            x if x == self.stay => 2,
            _ => panic!("Invalid distribution"),
        }
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.up, self.down, self.stay]
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct State {
    img: Image,
    infer: Inference,
}

impl State {
    pub fn new(img: Image, infer: Inference) -> State {
        State { img, infer }
    }
    pub fn to_tuple(&self) -> (Image, Inference) {
        (self.img.clone(), self.infer.clone())
    }
    pub fn get_image(&self) -> &Image {
        &self.img
    }
}

#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Lifecycle {
    Current,
    Unprocessed,
    Processed,
}

impl Lifecycle {
    pub fn new() -> Lifecycle {
        Lifecycle::Current
    }
}

impl From<&str> for Lifecycle {
    fn from(s: &str) -> Lifecycle {
        match s {
            "Current" => Lifecycle::Current,
            "Unprocessed" => Lifecycle::Unprocessed,
            "Processed" => Lifecycle::Processed,
            _ => panic!("Invalid lifecycle"),
        }
    }
}

impl ToString for Lifecycle {
    fn to_string(&self) -> String {
        match self {
            Lifecycle::Current => "Current".to_string(),
            Lifecycle::Unprocessed => "Unprocessed".to_string(),
            Lifecycle::Processed => "Processed".to_string(),
        }
    }
}

/// A representation of a full game
#[wasm_bindgen]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sequence {
    id: f64,
    /// The sequence of states
    sequence: Vec<State>,
    /// The outcome of the sequence
    outcome: Option<bool>,
    /// Lifecycle of the sequence <CURRENT, UNPROCESSED, PROCESSED>
    lifecycle: Lifecycle,
}

impl Sequence {
    pub fn new() -> Sequence {
        Sequence {
            id: 0.0,
            sequence: Vec::new(),
            outcome: None,
            lifecycle: Lifecycle::new(),
        }
    }
    pub fn new_with_id(id: f64) -> Sequence {
        Sequence {
            id,
            sequence: Vec::new(),
            outcome: None,
            lifecycle: Lifecycle::new(),
        }
    }
    pub fn get_outcome(&self) -> Option<bool> {
        self.outcome
    }
    pub fn len(&self) -> usize {
        self.sequence.len()
    }
    pub fn get_sequence(&self) -> &Vec<State> {
        &self.sequence
    }
}

impl Iterator for Sequence {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        self.sequence.pop()
    }
}

/// Initializes the indexedDB database
pub async fn init_db() -> Result<Rexie> {
    let rexie = Rexie::builder(DB_NAME)
        .version(2)
        .add_object_store(
            ObjectStore::new(STATE_STORE)
                .key_path(STATE_DB_KEY)
                .auto_increment(true)
                .add_index(Index::new(STATE_DB_KEY, STATE_DB_KEY).unique(true))
                .add_index(Index::new(STATE_STORE, STATE_STORE).unique(false)),
        )
        .add_object_store(
            ObjectStore::new(MODEL_STORE)
                .key_path_array([MODEL_DB_KEY])
                .auto_increment(false)
                .add_index(Index::new(MODEL_DB_KEY, MODEL_DB_KEY).unique(true)),
        )
        .build()
        .await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(MODEL_STORE)?;
    let model_data = store.get_all(None, None).await?;
    if model_data.is_empty() {
        let model = Model::new();
        match model.to_jsobject() {
            Ok(o) => {
                web_sys::console::log_1(&"Initializing DB with new model".into());
                web_sys::console::log_1(&o);
                store.put(&o.into(), None).await?;
            }
            Err(e) => {
                web_sys::console::log_1(&e.into());
            }
        };
    }
    transaction.done().await?;
    Ok(rexie)
}

pub async fn get_current_game() -> Result<Sequence> {
    let rexie = init_db().await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(STATE_STORE)?;
    let index = store.index(STATE_STORE)?;
    let state_js = index
        .get(JsValue::from(Lifecycle::Current.to_string()))
        .await?;
    if state_js.is_none() {
        web_sys::console::log_1(&"No current game found".into());
        return Ok(Sequence::new());
    }
    let state_js = state_js.unwrap();
    transaction.done().await?;
    Ok(
        match serde_wasm_bindgen::from_value::<Sequence>(state_js.clone()) {
            Ok(s) => s,
            Err(e) => {
                web_sys::console::log_1(&e.into());
                Sequence::new()
            }
        },
    )
}

/// Utility function to read a model from browser storage
pub async fn read_model() -> Result<Model> {
    let rexie = init_db().await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(MODEL_STORE)?;
    let key = Some(JsValue::from_f64(MODEL_DB_KEY_VERSION));

    let model_js = store.get(key.into()).await?;
    match Model::from_jsobject(model_js.into()) {
        Ok(m) => {
            transaction.done().await?;
            return Ok(m);
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    }
    Ok(Model::new())
}

/// Utility function to write a model to the browser storage
pub async fn write_model(model: Model) -> Result<()> {
    let rexie = init_db().await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(MODEL_STORE)?;
    match model.to_jsobject() {
        Ok(o) => {
            web_sys::console::log_1(&"Initializing DB with new model".into());
            web_sys::console::log_1(&o);
            store.put(&o.into(), None).await?;
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    };
    transaction.done().await?;
    Ok(())
}

/// Utility function to read a state from browser storage
pub async fn read_unprocessed_states() -> Result<Vec<Sequence>> {
    let rexie = init_db().await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(STATE_STORE)?;
    let keyrange = KeyRange::only(&Lifecycle::Unprocessed.into())?;
    let states_js = store.get_all(Some(keyrange), None).await?;
    transaction.done().await?;
    Ok(states_js
        .iter()
        .map(
            |state_js| match serde_wasm_bindgen::from_value::<Sequence>(state_js.into()) {
                Ok(s) => s,
                Err(e) => {
                    web_sys::console::log_1(&e.into());
                    Sequence::new()
                }
            },
        )
        .collect::<Vec<_>>())
}

/// Utility function to write an update to the browser storage
pub async fn write_new_state(state: Sequence) -> Result<()> {
    let rexie = init_db().await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(STATE_STORE)?;
    match serde_wasm_bindgen::to_value(&state) {
        Ok(o) => {
            store.add(&o, None).await?;
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    };
    transaction.done().await?;
    Ok(())
}

/// Adds a frame to the current game in browser storage
pub async fn add_frame(frame: State) -> Result<()> {
    let mut state = get_current_game().await?;
    let rexie = init_db().await?;
    state.sequence.push(frame);
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(STATE_STORE)?;
    match serde_wasm_bindgen::to_value(&state) {
        Ok(o) => {
            store.put(&o.into(), None).await?;
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    };
    transaction.done().await?;
    Ok(())
}

/// Dumps the current game to the history in browser storage
pub async fn end_game(outcome: bool) -> Result<()> {
    let mut state = get_current_game().await?;
    let rexie = init_db().await?;
    state.outcome = Some(outcome);
    state.lifecycle = Lifecycle::Unprocessed;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(STATE_STORE)?;
    let id = state.id;
    match serde_wasm_bindgen::to_value(&state) {
        Ok(o) => {
            store.put(&o.into(), None).await?;
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    };

    let new_state = Sequence::new_with_id(id + 1.0);
    match serde_wasm_bindgen::to_value(&new_state) {
        Ok(o) => {
            store.put(&o.into(), None).await?;
        }
        Err(e) => {
            web_sys::console::log_1(&e.into());
        }
    };
    transaction.done().await?;
    Ok(())
}
