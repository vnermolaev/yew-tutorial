//import livereload from "rollup-plugin-livereload"

export default {
  input: "main.js",
  output: {
    file: "app/bundle.js",
    format: "iife",
  }
//  ,plugins: [livereload("build/wasm")],
};