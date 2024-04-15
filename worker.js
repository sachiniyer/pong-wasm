importScripts("./pkg/pong_wasm.js");

console.log("Initializing worker");
const { Model, State } = wasm_bindgen;

const ACTION = {
  up: 1.0,
  down: -1.0,
  stay: 0.0,
};
const QUADRANTS = 100;

function location() {
  let xw = canvas.width / QUADRANTS;
  let xpos = ball.x / xw;

  let yw = canvas.height / QUADRANTS;
  let ypos = ball.y / yw;

  return State.new(xpos, ypos);
}

async function update_loop(model) {
  model.update(location(), ACTION.up);
}

async function initialize() {
  await wasm_bindgen("./pkg/pong_wasm_bg.wasm");
  console.log("Worker Initialized");

  var model = Model.new();
  model.update(State.new(0.0, 0.0), ACTION.up);
}

initialize();
