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
    pub hidden: Vec<f32>,
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
                hidden: h1.flatten_all()?.to_vec1()?,
            })
        };
        infer_wrapper().unwrap_or_else(|e| {
            web_sys::console::error_1(&e.to_string().into());
            Inference {
                dist: Distribution::new(0.0, 0.0, 0.0),
                choice: 0,
                hidden: Vec::new(),
            }
        })
    }
    pub fn train(&mut self, seq: &Sequence) {}
}
