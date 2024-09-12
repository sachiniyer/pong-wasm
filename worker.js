importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");
const { Model, handle_img, new_image, startup } = wasm_bindgen;

const ACTION = {
  up: 1.0,
  down: -1.0,
  stay: 0.0,
};

async function initialize() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");
  console.log("Worker Initialized");
  self.postMessage(JSON.stringify({ topic: "ping-wasm", data: "pong" }));
}

async function send_state(state) {
  let data = state;
  let dim = state.length;
  let choice = await handle_img(new_image(data.flat()));
  console.log(0);
}

// // Can be used to display the state in the console
// function display_state(state) {
//   let dim = state.length;
//   let str = "";
//   for (let i = 0; i < dim; i++) {
//     for (let j = 0; j < dim; j++) {
//       if (state[i][j]) {
//         str += "X";
//       } else {
//         str += " ";
//       }
//     }
//     str += "\n";
//   }
// }

self.onmessage = async (e) => {
  self.postMessage({ type: "pong", message: "pong" });
  switch (e.data.topic) {
    case "ping":
      self.postMessage({ type: "pong", message: "pong" });
      break;
    case "state":
      await send_state(e.data.data);
      break;
    default:
      break;
  }
};

initialize();
