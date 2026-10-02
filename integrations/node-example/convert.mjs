import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { convert } = require("./pkg/nlp_engine_ind_wasm.js");

const text = process.argv[2] ?? "I'll call you later";
const language = process.argv[3] ?? "kn";

const result = convert(text, language);
console.log(JSON.stringify(result, null, 2));
