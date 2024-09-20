use crate::{
    consts::QUADRANTS,
    state::{Distribution, Image, Sequence},
};

use candle_core::{Device, Tensor};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Object, Reflect};

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Model {
    id: u8,
    w1: Tensor,
    w2: Tensor,
    val: bool,
}

#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct ModelSerializer {
    id: u8,
    w1: Vec<f32>,
    w2: Vec<f32>,
    val: bool,
}

#[derive(Clone, Debug)]
#[wasm_bindgen]
pub struct Inference {
    pub dist: Distribution,
    pub choice: u8,
}

/// The implementation of the model.
/// The model is a simple neural network with two layers.
///
/// RL model:
/// - input: QUADRANTS x QUADRANTS matrix
/// - output: P(UP), P(DOWN), P(STAY)
/// - loss: cross-entropy
///
/// The model is trained using Policy Gradient method.
#[wasm_bindgen]
impl Model {
    pub fn new() -> Model {
        let device = Device::Cpu;
        Model {
            id: 0,
            val: false,
            w1: Tensor::randn(0f32, 1.0, (QUADRANTS, QUADRANTS), &device).unwrap_throw(),
            w2: Tensor::randn(0f32, 1.0, (QUADRANTS, QUADRANTS), &device).unwrap_throw(),
        }
    }

    pub fn from_jsobject(model: JsValue) -> Result<Model, serde_wasm_bindgen::Error> {
        let device = Device::Cpu;
        let model: ModelSerializer = serde_wasm_bindgen::from_value(model)?;
        Ok(Model {
            id: model.id,
            val: model.val,
            w1: Tensor::from_vec(model.w1, (QUADRANTS, QUADRANTS), &device).unwrap_throw(),
            w2: Tensor::from_vec(model.w2, (QUADRANTS, QUADRANTS), &device).unwrap_throw(),
        })
    }

    pub fn infer(&self, img: Image) -> Inference {
        Inference {
            dist: Distribution::new(0.0, 0.0, 1.0),
            choice: 1,
        }
    }

    pub fn train(&self, seq: Sequence) {}

    pub fn to_jsobject(&self) -> Result<Object, JsValue> {
        let w1: Vec<f32> = self
            .w1
            .to_vec2()
            .unwrap_throw()
            .into_iter()
            .flatten()
            .collect();

        let w2: Vec<f32> = self
            .w2
            .to_vec2()
            .unwrap_throw()
            .into_iter()
            .flatten()
            .collect();

        let object = Object::new();
        Reflect::set(&object, &"id".into(), &JsValue::from(self.id))?;
        Reflect::set(&object, &"w1".into(), &JsValue::from(w1))?;
        Reflect::set(&object, &"w2".into(), &JsValue::from(w2))?;
        Reflect::set(&object, &"val".into(), &JsValue::from(self.val))?;
        Ok(object)
    }
}
