use crate::{
    consts::{HIDDEN, QUADRANTS, RESOLUTION},
    state::{Distribution, Image, Sequence},
};

use candle_core::{DType, Device, Tensor};
use candle_nn::ops::softmax;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Object, Reflect};

#[derive(Clone, Debug)]
pub struct Model {
    id: u8,
    w1: Tensor,
    w2: Tensor,
    val: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ModelSerializer {
    id: u8,
    w1: Vec<f32>,
    w2: Vec<f32>,
    val: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Inference {
    pub dist: Distribution,
    pub choice: u8,
    pub hidden: String,
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
impl Model {
    pub fn new() -> Model {
        let device = Device::Cpu;
        Model {
            id: 0,
            val: false,
            w1: Tensor::randn(
                0f32,
                1.0,
                ((QUADRANTS / RESOLUTION) * (QUADRANTS / RESOLUTION), HIDDEN),
                &device,
            )
            .unwrap_throw(),
            w2: Tensor::randn(0f32, 1.0, (HIDDEN, 3), &device).unwrap_throw(),
        }
    }

    pub fn from_jsobject(model: JsValue) -> Result<Model, serde_wasm_bindgen::Error> {
        let device = Device::Cpu;
        let model: ModelSerializer = serde_wasm_bindgen::from_value(model)?;
        Ok(Model {
            id: model.id,
            val: model.val,
            w1: Tensor::from_vec(model.w1, (QUADRANTS, HIDDEN), &device).unwrap_or_else(|e| {
                web_sys::console::error_1(&e.to_string().into());
                Tensor::randn(0f32, 1.0, (QUADRANTS, HIDDEN), &device).unwrap_throw()
            }),
            w2: Tensor::from_vec(model.w2, (HIDDEN, 3), &device).unwrap_or_else(|e| {
                web_sys::console::error_1(&e.to_string().into());
                Tensor::randn(0f32, 1.0, (HIDDEN, 3), &device).unwrap_throw()
            }),
        })
    }

    // the proper way to do this is the same deep js object creation we do for model...
    // but, I'm lazy
    pub fn serialize_hidden(tensor: &Tensor) -> Result<String, candle_core::Error> {
        Ok(tensor
            .flatten_all()?
            .to_vec1::<f32>()?
            .iter()
            .map(|x| format!("{}", x))
            .collect::<Vec<_>>()
            .join(","))
    }

    pub fn deserialize_hidden(hidden: &str) -> Result<Tensor, candle_core::Error> {
        let hidden = hidden
            .split(",")
            .map(|x| x.parse::<f32>().unwrap_throw())
            .collect::<Vec<_>>();
        Ok(Tensor::from_vec(hidden, (1, HIDDEN), &Device::Cpu)?)
    }

    pub fn to_jsobject(&self) -> Result<Object, JsValue> {
        let to_jsobject_wrapper = || -> Result<(Vec<f32>, Vec<f32>), candle_core::Error> {
            let w1: Vec<f32> = self.w1.to_vec2()?.into_iter().flatten().collect();
            let w2: Vec<f32> = self.w2.to_vec2()?.into_iter().flatten().collect();
            Ok((w1, w2))
        };
        let (w1, w2) = to_jsobject_wrapper().unwrap_or_else(|e| {
            web_sys::console::error_1(&e.to_string().into());
            (Vec::new(), Vec::new())
        });
        let object = Object::new();
        Reflect::set(&object, &"id".into(), &JsValue::from(self.id))?;
        Reflect::set(&object, &"w1".into(), &JsValue::from(w1))?;
        Reflect::set(&object, &"w2".into(), &JsValue::from(w2))?;
        Reflect::set(&object, &"val".into(), &JsValue::from(self.val))?;
        Ok(object)
    }

    // https://karpathy.github.io/2016/05/31/rl/
    pub fn infer(&self, img: Image) -> Inference {
        let infer_wrapper = || -> Result<Inference, candle_core::Error> {
            let input = Tensor::from_vec(
                img,
                (1, (QUADRANTS / RESOLUTION) * (QUADRANTS / RESOLUTION)),
                &Device::Cpu,
            )?
            .to_dtype(DType::F32)?;
            let h1 = input.matmul(&self.w1)?.relu()?;
            let h2 = h1.matmul(&self.w2)?;
            let p = softmax(&h2, 1)?.flatten_all()?.to_vec1::<f32>()?;
            let dist = Distribution::new(p[0], p[1], p[2]);
            let choice = dist.sample();
            Ok(Inference {
                dist,
                choice,
                hidden: Model::serialize_hidden(&h1)?,
            })
        };
        infer_wrapper().unwrap_or_else(|e| {
            web_sys::console::error_1(&e.to_string().into());
            Inference {
                dist: Distribution::new(0.0, 0.0, 0.0),
                choice: 0,
                hidden: String::new(),
            }
        })
    }

    pub fn train(&mut self, seq: &Sequence) {
        // grab all the states
        // create the rewards for each of the states
        // - was it a win or loss
        // - how far in the cycle did it happen (discounted reward)
        // modulate the gradients based on the discounted rewards (multiply)
        // run backpropagation with the hidden states and the modulated gradients
        // update the weights
        // repeat until all states are trained on
        let mut train_wrapper = || -> Result<(), candle_core::Error> {
            let mut rewards = vec![0.0; seq.len()];
            for i in (0..seq.len()).rev() {
                let mut discounted_reward;
                if seq.get_outcome().unwrap_throw() {
                    discounted_reward = 1.0;
                } else {
                    discounted_reward = 0.0;
                }
                discounted_reward = 0.99 * discounted_reward;
                rewards[i] = discounted_reward;
            }
            web_sys::console::log_1(
                &(rewards
                    .clone()
                    .into_iter()
                    .reduce(|acc, i| (acc + i))
                    .unwrap()
                    / rewards.len() as f32)
                    .to_string()
                    .into(),
            );
            for i in 0..seq.len() {
                let state = &seq.get_sequence()[i];
                let reward = rewards[i];
                let (image, inference) = state.to_tuple();
                let choice = inference.choice;
                let hidden = Model::deserialize_hidden(&inference.hidden)?;
                let dist = inference.dist.to_vec();
                let mut d_h2 = [0.0; 3];
                d_h2[0] = (dist[0] - if choice == 0 { 1.0 } else { 0.0 }) * reward;
                d_h2[1] = (dist[1] - if choice == 1 { 1.0 } else { 0.0 }) * reward;
                d_h2[2] = (dist[2] - if choice == 2 { 1.0 } else { 0.0 }) * reward;
                let d_h2 = Tensor::from_vec(d_h2.to_vec(), (1, 3), &Device::Cpu)?;
                let d_w2 = hidden.t()?.matmul(&d_h2)?;
                let d_h1 = d_h2.matmul(&self.w2.t()?)?;
                let d_w1 = Tensor::from_vec(
                    image.to_vec(),
                    (1, (QUADRANTS / RESOLUTION) * (QUADRANTS / RESOLUTION)),
                    &Device::Cpu,
                )?
                .to_dtype(DType::F32)?
                .t()?
                .matmul(&d_h1)?;
                self.w1 = self.w1.sub(&d_w1)?;
                self.w2 = self.w2.sub(&d_w2)?;
            }
            Ok(())
        };

        train_wrapper().unwrap_or_else(|e| web_sys::console::error_1(&e.to_string().into()));
    }
}
