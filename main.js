"use strict";
const fs = require("fs");
const { WASI } = require("wasi");
const wasi = new WASI({
  args: process.argv,
  env: process.env,
  preopens: {
    "/sandbox": process.cwd()
  }
});
const importObject = { wasi_snapshot_preview1: wasi.wasiImport };
(async () => {
  const wasm = await WebAssembly.compile(
    fs.readFileSync("target/wasm32-wasi/debug/autorust.wasm")
  );
  const instance = await WebAssembly.instantiate(wasm, importObject);
  wasi.start(instance);
})();
