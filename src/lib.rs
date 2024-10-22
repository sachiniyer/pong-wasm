pub mod consts;
pub mod model;
pub mod state;

use crate::state::{add_frame, end_game, read_model, read_unprocessed_states, write_model, State};

use rexie::Error;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, MessageEvent, Worker};

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    topic: String,
    data: String,
}

#[wasm_bindgen]
pub fn startup() {
    let worker_handle = Rc::new(RefCell::new(Worker::new("./worker.js").unwrap()));
    #[allow(unsafe_code)]
    console::log_1(&"Created a new worker from within Wasm".into());

    let closure = Closure::wrap(Box::new(move |event: MessageEvent| {
        let data = event.data().as_string().unwrap();
        web_sys::console::log_1(&format!("Message from worker: {}", data).into());
        let data = serde_json::from_str::<Event>(&data.clone()).unwrap();
        web_sys::console::log_1(&format!("Deserialized data: {:?}", data).into());
    }) as Box<dyn FnMut(MessageEvent)>);

    worker_handle
        .borrow()
        .set_onmessage(Some(closure.as_ref().unchecked_ref()));
    closure.forget(); // Keep the closure alive
}

#[wasm_bindgen]
pub async fn handle_img(img: Vec<u8>, save: bool) -> u8 {
    let handle_img_wrapper = async {
        let model = read_model().await.unwrap_or_else(|e| {
            web_sys::console::log_1(&format!("{:?}", e).into());
            model::Model::new()
        });
        let inference = model.infer(img.clone());
        let inference_choice = inference.choice;
        if save {
            add_frame(State::new(img, inference))
                .await
                .unwrap_or_else(|e| {
                    web_sys::console::log_1(&format!("{:?}", e).into());
                });
        }
        Ok::<u8, Error>(inference_choice)
    };
    match handle_img_wrapper.await {
        Ok(choice) => choice,
        Err(e) => {
            web_sys::console::log_1(&format!("{:?}", e).into());
            0
        }
    }
}

#[wasm_bindgen]
pub async fn handle_end(outcome: bool) {
    let train_wrapper = async {
        end_game(outcome).await.unwrap_throw();
        let mut model = read_model().await.unwrap_or_else(|e| {
            web_sys::console::log_1(&format!("{:?}", e).into());
            model::Model::new()
        });
        let unprocessed_states = read_unprocessed_states().await.unwrap_or_else(|e| {
            web_sys::console::log_1(&format!("{:?}", e).into());
            vec![]
        });
        unprocessed_states.iter().for_each(|state| {
            model.train(state);
        });
        let _ = write_model(model).await.unwrap_or_else(|e| {
            web_sys::console::log_1(&format!("{:?}", e).into());
        });
        Ok::<(), Error>(())
    };
    match train_wrapper.await {
        Ok(_) => {
            web_sys::console::log_1(&"Training successful".into());
        }
        Err(e) => {
            web_sys::console::log_1(&format!("{:?}", e).into());
        }
    }
}
