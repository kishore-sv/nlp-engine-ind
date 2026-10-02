import init, { convert, supported_languages } from "./pkg/nlp_engine_ind_wasm.js";

const input = document.getElementById("input");
const output = document.getElementById("output");
const meta = document.getElementById("meta");
const language = document.getElementById("language");
const button = document.getElementById("convert");

let ready = false;

async function boot() {
  await init();
  ready = true;
  output.textContent = "Ready. Supported: " + supported_languages().join(", ");
}

button.addEventListener("click", async () => {
  if (!ready) return;
  const result = convert(input.value, language.value);
  output.textContent = result.output;
  meta.textContent = `confidence=${result.confidence.toFixed(2)} latency=${result.processing_time_us}µs`;
});

boot().catch((err) => {
  output.textContent = "Failed to load WASM. Run ./scripts/build-wasm.sh";
  meta.textContent = String(err);
});
