importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");
const { Model, handle_img, startup, handle_end } = wasm_bindgen;

const DEBUG = false;
let mode = "train";
async function initialize() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");
  console.log("Worker Initialized");
  self.postMessage(JSON.stringify({ topic: "ping-wasm", data: "pong" }));
}

async function send_state(state) {
  if (mode == "human") {
    return;
  }
  if (mode == "play") {
    let data = state;
    let choice = await handle_img(data.flat(), true);
    self.postMessage({ type: "movePlayer1", data: choice });
  }
  if (mode == "train") {
    let data = state;
    let choice = await handle_img(data.flat(), true);
    self.postMessage({ type: "movePlayer1", data: choice });
    let data2 = state.map((row) => row.slice().reverse());
    let choice2 = await handle_img(data2.flat(), false);
    self.postMessage({ type: "movePlayer2", data: choice2 });
  }
}

async function send_end(outcome) {
  await handle_end(outcome);
}

function display_state(state) {
  let dim = state.length;
  let str = "";
  for (let i = 0; i < dim; i++) {
    for (let j = 0; j < dim; j++) {
      if (state[i][j]) {
        str += "X";
      } else {
        str += " ";
      }
    }
    str += "\n";
  }
}

self.onmessage = async (e) => {
  self.postMessage({ type: "pong", message: "pong" });
  switch (e.data.type) {
    case "ping":
      self.postMessage({ type: "pong", message: "pong" });
      break;
    case "state":
      if (DEBUG) {
        display_state(e.data.data);
      }
      await send_state(e.data.data);
      break;
    case "end":
      await send_end(e.data.data);
      break;
    case "mode":
      mode = e.data.data;
      break;
    default:
      break;
  }
};

initialize();
