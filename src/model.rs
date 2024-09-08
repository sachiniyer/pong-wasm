use crate::{
    consts::QUADRANTS,
    state::{Distribution, Image, Sequence},
};

use candle_core::{Device, Tensor};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone)]
#[wasm_bindgen]
pub struct Model {
    id: u8,
    w1: Tensor,
    w2: Tensor,
    val: bool,
}

#[derive(Clone)]
#[wasm_bindgen]
pub struct Inference {
    pub dist: Distribution,
    pub choice: u8,
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let id: u8 = self.id;
        let w1: Vec<f64> = self.w1.to_vec1().unwrap_throw();
        let w2: Vec<f64> = self.w2.to_vec1().unwrap_throw();
        let val = self.val;
        let model = (id, w1, w2, val);
        model.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Model {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Model, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let device = Device::Cpu;
        let (id, w1, w2, val): (u8, Vec<f64>, Vec<f64>, bool) =
            Deserialize::deserialize(deserializer)?;
        let w1 = Tensor::from_vec(w1, (QUADRANTS, QUADRANTS), &device).unwrap_throw();
        let w2 = Tensor::from_vec(w2, (QUADRANTS, QUADRANTS), &device).unwrap_throw();
        Ok(Model { id, w1, w2, val })
    }
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

    pub fn infer(&self, img: Image) -> Inference {
        Inference {
            dist: Distribution::new(0.0, 0.0, 1.0),
            choice: 1,
        }
    }

    pub fn train(&self, seq: Sequence) {}
}
