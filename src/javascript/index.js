import init, { greet } from "../../pkg/wasm_hello_world.js";

async function run() {
  await init();
  const message = greet('rawr');
  document.getElementById('output').innerHTML = message;
}

run();
