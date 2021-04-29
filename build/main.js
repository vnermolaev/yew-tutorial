import init from "./app/wasm.js"
import { __compiled } from "./js/version.js"

// Add compilation time
console.info("WASM program compiled: " + __compiled())

init()