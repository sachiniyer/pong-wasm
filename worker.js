importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");
const { Model, State, handle_state } = wasm_bindgen;

const ACTION = {
  up: 1.0,
  down: -1.0,
  stay: 0.0,
};

async function initialize() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");
  console.log("Worker Initialized");

  var model = Model.new();
  model.update(State.new(0.0, 0.0), ACTION.up);
}

initialize();

self.onmessage = async (e) => {
  handle_state(String(e.data));

  self.postMessage({ type: "pong", message: "pong" });
};
