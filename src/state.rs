use crate::model::Model;
use crate::consts::{ DB_NAME, MODEL_STORE, STATE_STORE, MODEL_DB_KEY, MODEL_DB_KEY_VERSION };

use serde::{Deserialize, Serialize};

use wasm_bindgen::prelude::*;
use rexie::*;

pub type Image = Vec<u8>;

#[wasm_bindgen]
#[derive(Copy, Clone, Deserialize, Serialize)]
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

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize)]
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

#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize)]
pub enum Lifecycle {
    Current,
    Unprocessed,
    Processed,
}

impl Lifecycle {
    pub fn new() -> Lifecycle {
        Lifecycle::Unprocessed
    }
}

impl From<&str> for Lifecycle {
    fn from(s: &str) -> Lifecycle {
        match s {
            "current" => Lifecycle::Current,
            "unprocessed" => Lifecycle::Unprocessed,
            "processed" => Lifecycle::Processed,
            _ => panic!("Invalid lifecycle")
        }
    }
}

impl ToString for Lifecycle {
    fn to_string(&self) -> String {
        match self {
            Lifecycle::Current => "current".to_string(),
            Lifecycle::Unprocessed => "unprocessed".to_string(),
            Lifecycle::Processed => "processed".to_string(),
        }
    }
}

/// A representation of a full game
#[wasm_bindgen]
#[derive(Clone, Deserialize, Serialize)]
pub struct Sequence {
    id: f64,
    /// The sequence of states
    sequence: Vec<State>,
    /// The outcome of the sequence
    outcome: Option<bool>,
    /// Lifecycle of the sequence <CURRENT, UNPROCESSED, PROCESSED>
    lifecycle: Lifecycle,
}

pub async fn init_model_db() -> Result<()> {
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    if rexie.store_names().contains(&MODEL_STORE.to_string()) {
        return Ok(());
    }
    let rexie = Rexie::builder(DB_NAME)
       .version(1)
       .add_object_store(
           ObjectStore::new(MODEL_STORE)
               .key_path(MODEL_DB_KEY)
       )
       .build()
       .await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(MODEL_STORE)?;
    let model = Model::new();
    let model_js = serde_wasm_bindgen::to_value(&model).unwrap();
    let key = Some(JsValue::from_f64(MODEL_DB_KEY_VERSION));
    store.add(&model_js, key.as_ref()).await?;
    return Ok(());
}

pub async fn init_model_state() -> Result<()> {
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    if rexie.store_names().contains(&STATE_STORE.to_string()) {
        return Ok(());
    }
    Rexie::builder(DB_NAME)
       .version(1)
       .add_object_store(
           ObjectStore::new(STATE_STORE)
               .key_path("id")
               .auto_increment(true)
               .add_index(Index::new("lifecycle", "lifecycle").unique(false))
       )
       .build()
       .await?;
    Ok(())
}

pub async fn get_current_game() -> Result<Sequence> {
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(STATE_STORE)?;
    let keyrange = KeyRange::only(&Lifecycle::Current.into())?;
    let state_js = store.get_all(Some(keyrange), None).await?;
    Ok(serde_wasm_bindgen::from_value::<Sequence>(state_js.first().into()).unwrap())
}

/// Utility function to read a model from browser storage
pub async fn read_model() -> Result<Model> {
    init_model_db().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(MODEL_STORE)?;
    let key = Some(JsValue::from_f64(MODEL_DB_KEY_VERSION));
    let model_js = store.get(key.into()).await?;
    let model: Model = serde_wasm_bindgen::from_value(model_js.into()).unwrap();
    Ok(model)
}

/// Utility function to write a model to the browser storage
pub async fn write_model(model: Model) -> Result<()> {
    init_model_db().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    let transaction = rexie.transaction(&[MODEL_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(MODEL_STORE)?;
    let model_js = serde_wasm_bindgen::to_value(&model).unwrap();
    let key = Some(JsValue::from_f64(MODEL_DB_KEY_VERSION));
    store.put(&model_js, key.as_ref()).await?;
    Ok(())
}

/// Utility function to read a state from browser storage
pub async fn read_unprocessed_states() -> Result<Vec<Sequence>> {
    init_model_state().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadOnly)?;
    let store = transaction.store(STATE_STORE)?;
    let keyrange = KeyRange::only(&Lifecycle::Unprocessed.into())?;
    let states_js = store.get_all(Some(keyrange), None).await?;
    Ok(states_js.iter().map(|state_js| {
        serde_wasm_bindgen::from_value::<Sequence>(state_js.into()).unwrap()
    }).collect::<Vec<_>>())
}

/// Utility function to write an update to the browser storage
pub async fn write_new_state(state: Sequence) -> Result<()> {
    init_model_state().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite).unwrap();
    let store = transaction.store(STATE_STORE).unwrap();
    let state_js = serde_wasm_bindgen::to_value(&state).unwrap();
    store.add(&state_js, None).await?;
    Ok(())
}

/// Adds a frame to the current game in browser storage
pub async fn add_frame(frame: State) -> Result<()> {
    init_model_state().await?;
    let mut state = get_current_game().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    state.sequence.push(frame);
    let state_js = serde_wasm_bindgen::to_value(&state).unwrap();
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite)?;
    let store = transaction.store(STATE_STORE)?;
    store.put(&state_js, Some(&JsValue::from(state.id))).await?;
    Ok(())
}

/// Dumps the current game to the history in browser storage
pub async fn end_game(outcome: bool) -> Result<()>{
    init_model_state().await?;
    let mut state = get_current_game().await?;
    let rexie = Rexie::builder(DB_NAME)
        .build()
        .await?;
    state.outcome = Some(outcome);
    let state_js = serde_wasm_bindgen::to_value(&state).unwrap();
    let transaction = rexie.transaction(&[STATE_STORE], TransactionMode::ReadWrite).unwrap();
    let store = transaction.store(STATE_STORE).unwrap();
    store.put(&state_js, Some(&JsValue::from(state.id))).await?;
    Ok(())
}
