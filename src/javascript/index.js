import init, { Integration } from "../../pkg/wasm_rust_playground.js";

async function run() {
  await init();
  const integration = new Integration();

  function tick () {
    integration.tick();
    requestAnimationFrame(tick)
  }
  tick()
}

run();
