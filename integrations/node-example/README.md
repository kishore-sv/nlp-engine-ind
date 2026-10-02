# Node.js example

Uses the same WASM build as the web example — no duplicate NLP logic.

## Prerequisites

```bash
./scripts/build-wasm.sh
```

## Run

```bash
cd integrations/node-example
node convert.mjs "I'll call you later" kn
```

Output:

```json
{
  "output": "...",
  "confidence": 0.85,
  "processing_time_us": 1234,
  "language": "kn"
}
```

## Integration

Import from `./pkg/` in this directory (built by `./scripts/build-wasm.sh`).
