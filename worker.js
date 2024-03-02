importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");

const { Model, State } = wasm_bindgen;

async function init_wasm_in_worker() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");

  var model = Model.new();
  model.update(State.new(), true);
  console.log("Updated");

  self.onmessage = async (event) => {
    var worker_result = model.direction({});

    self.postMessage(worker_result);
  };
}

init_wasm_in_worker();
