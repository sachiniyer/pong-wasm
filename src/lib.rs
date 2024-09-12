pub mod consts;
pub mod model;
pub mod state;

use crate::state::{read_model, Image, State};

use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use web_sys::{console, MessageEvent, Worker};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    topic: String,
    data: String,
}

/*
 * PLAN:
 * - Use handle_state to take data for RL model
 * - Store weights in browser Storage (localStorage)
 * - On inference call, take state and publish results.
 */
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
pub async fn handle_img(img: Image) -> u8 {
    // infer with weights that you take from local storage here.
    let model = read_model().await;
    match model {
        Ok(m) => {
            web_sys::console::log_1(&m.into());
        }
        Err(e) => web_sys::console::log_1(&format!("{:?}", e).into()),
    }
    // let inference = model.infer(img.clone());
    // add_frame(State::new(img, inference.dist, inference.choice));
    0
}

// #[wasm_bindgen]
// pub fn handle_end(outcome: bool) {
//     let model = read_model();
//     model.train(dump_game(outcome));
//     write_model(model);
// }
