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
            w1: Tensor::from_vec(model.w1, (QUADRANTS, HIDDEN), &device).unwrap_throw(),
            w2: Tensor::from_vec(model.w2, (HIDDEN, 3), &device).unwrap_throw(),
        })
    }

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

    // https://karpathy.github.io/2016/05/31/rl/
    pub fn infer(&self, img: Image) -> Inference {
        let input = Tensor::from_vec(
            img,
            (1, (QUADRANTS / RESOLUTION) * (QUADRANTS / RESOLUTION)),
            &Device::Cpu,
        )
        .unwrap_throw()
        .to_dtype(DType::F32)
        .unwrap_throw();
        let h1 = input.matmul(&self.w1).unwrap_throw().relu().unwrap_throw();
        let h2 = h1.matmul(&self.w2).unwrap_throw();
        let p = softmax(&h2, 1)
            .unwrap_throw()
            .flatten_all()
            .unwrap_throw()
            .to_vec1::<f32>()
            .unwrap_throw();
        let dist = Distribution::new(p[0], p[1], p[2]);
        let choice = dist.sample();
        Inference { dist, choice }
    }

    pub fn train(&mut self, seq: Sequence) {
        // let device = Device::Cpu;
        // let mut grad_w1 = Tensor::zeros((QUADRANTS, HIDDEN), DType::F32, &device).unwrap_throw();
        // let mut grad_w2 = Tensor::zeros((HIDDEN, 3), DType::F32, &device).unwrap_throw();
        // let mut reward = 0.0;
        // let mut logp = 0.0;
        // let outcome = seq.get_outcome();
        // let seqlen = seq.len();
        // for state in seq.into_iter() {
        //     let (img, action, reward_) = state.to_tuple();
        //     let input = Tensor::from_vec(img, (QUADRANTS, QUADRANTS), &device).unwrap_throw();
        //     let h = self.w1.matmul(&input).unwrap_throw().relu().unwrap_throw();
        //     let h2 = self.w2.matmul(&h).unwrap_throw();
        //     let logp_ = log_softmax(&h2, 0).unwrap_throw();
        //     logp += logp_
        //         .get(action.choice() as usize)
        //         .unwrap_throw()
        //         .to_vec0::<f32>()
        //         .unwrap_throw();
        //     reward += reward_ as f32;
        //     let mut dlogp = Vec::new();
        //     for i in 0..3 {
        //         dlogp.push(if i == action.choice() as usize {
        //             1.0 - logp_.get(i).unwrap_throw().to_vec0::<f32>().unwrap_throw()
        //         } else {
        //             -logp_.get(i).unwrap_throw().to_vec0::<f32>().unwrap_throw()
        //         });
        //     }
        //     let dlogp = Tensor::from_vec(dlogp, (3,), &device).unwrap_throw();
        //     grad_w2 = dlogp
        //         .matmul(&h.t().unwrap_throw())
        //         .unwrap_throw()
        //         .add(&grad_w2)
        //         .unwrap_throw();
        //     grad_w1 = self
        //         .w2
        //         .t()
        //         .unwrap_throw()
        //         .matmul(&dlogp)
        //         .unwrap_throw()
        //         .matmul(&input.t().unwrap_throw())
        //         .unwrap_throw()
        //         .add(&grad_w1)
        //         .unwrap_throw();
        // }
        // let reward = reward / seqlen as f32;
        // let loss = -logp * reward;
        // let lr = 0.01;
        // self.w1 = grad_w1
        //     .mul(&Tensor::from_vec(vec![lr], (1,), &device).unwrap_throw())
        //     .unwrap_throw()
        //     .sub(&self.w1)
        //     .unwrap_throw();
        // self.w2 = grad_w2
        //     .mul(&Tensor::from_vec(vec![lr], (1,), &device).unwrap_throw())
        //     .unwrap_throw()
        //     .sub(&self.w2)
        //     .unwrap_throw();
    }
}
