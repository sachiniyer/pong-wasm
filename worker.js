importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");

const { NumberEval } = wasm_bindgen;

async function init_wasm_in_worker() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");

  var num_eval = NumberEval.new();

  self.onmessage = async (event) => {
    var worker_result = num_eval.is_even(event.data);

    self.postMessage(worker_result);
  };
}

init_wasm_in_worker();
