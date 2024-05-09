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
}

function send_state(state) {
  let data = state;
  let dim = state.length;
  handle_state(State.new(data.flat(), dim));
}

self.onmessage = async (e) => {
  switch (e.data.type) {
    case "ping":
      self.postMessage({ type: "pong", message: "pong" });
      break;
    case "state":
      send_state(e.data.state);
      break;
    default:
      break;
  }
};

initialize();
