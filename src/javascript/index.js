import init, { Integration } from "../../pkg/wasm_rust_playground.js"

const { min, max } = Math

async function run() {
  await init()
  const integration = new Integration()

  let previousTime = 0
  function tick (timems) {
    const time = timems / 1000
    const dt = clamp(time - previousTime, 0, 0.0333)

    integration.tick(dt)
    requestAnimationFrame(tick)

    previousTime = time
  }
  requestAnimationFrame(tick)
}

run();

function clamp (value, lower, upper) {
  return min(max(value, lower), upper)
}
