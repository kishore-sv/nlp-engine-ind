# WebAssembly integration

## Build

```bash
./scripts/build-wasm.sh
```

Output: `integrations/web-example/pkg/`

## Browser API

```javascript
import init, { convert, convert_json, supported_languages } from "./pkg/nlp_engine_ind_wasm.js";

await init();

const result = convert("I'll call you later", "kn");
// { output, confidence, processing_time_us, language }

const jsonResult = convert_json(JSON.stringify({
  text: "I'll call you later",
  language: "kn",
  style: "casual",
  codeMix: "natural"
}));
```

## Web example

```bash
./scripts/build-wasm.sh
cd integrations/web-example
python3 -m http.server 8080
# Open http://localhost:8080
```

No server-side NLP — WASM runs entirely in the browser.

## Node.js

See `integrations/node-example/` for a Node.js wrapper using the same WASM package.

```bash
cd integrations/node-example
npm install
node convert.mjs "I'll call you later" kn
```

## Supported language codes

`kn`, `hi`, `te`, `ta`, `ml`

## Memory

WASM module loads all compiled languages. For smaller bundles, build with selective Cargo features and rebuild WASM.
