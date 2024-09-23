pub mod consts;
pub mod model;
pub mod state;

use crate::state::{read_model, add_frame, end_game, State};

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, MessageEvent, Worker};

#[derive(Serialize, Deserialize, Debug)]
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
        // serialize data to Event struct
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
    let model = read_model().await;
    match model {
        Ok(m) => {
            let inference = m.infer(img.clone());
            if !save {
                return inference.choice;
            }
            let add_frame_result = add_frame(State::new(img, inference.dist, inference.choice)).await;
            if add_frame_result.is_err() {
                web_sys::console::log_1(&format!("{:?}", add_frame_result).into());
            }
            return inference.choice;
        }
        Err(e) => {web_sys::console::log_1(&format!("{:?}", e).into());},
    }
    0 // something went wrong to get here -> but avoid panics
}

#[wasm_bindgen]
pub async fn handle_end(outcome: bool) {
    // let model = read_model();
    // model.train(dump_game(outcome));
    // write_model(model);
    match end_game(outcome).await {
       Ok(_) => web_sys::console::log_1(&"Game ended successfully".into()),
        Err(e) => web_sys::console::log_1(&format!("{:?}", e).into()),
    }
}
