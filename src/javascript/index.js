import init, { MovingShape } from "../../pkg/wasm_rust_playground.js";

async function run() {
  await init();
  const shape = new MovingShape();
  shape.render();
}

run();
