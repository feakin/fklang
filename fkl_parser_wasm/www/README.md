# Minimal Webpack 5 Raw WebAssembly Demo

Webpack 5 demo of WebAssembly with the most minimal configuration.

```
yarn
yarn webpack serve
```

Navigate to `localhost:9000`, open console and see a native WebAssembly module running.

You should see the following output in your browsers console:

```
[HMR] Waiting for update signal from WDS...
index.js:519 [webpack-dev-server] Hot Module Replacement enabled.
index.js:519 [webpack-dev-server] Live Reloading enabled.
index.js:8 ---- Sync Wasm Module
index.js:10 ƒ $_Z4facti() { [native code] }
index.js:11 1
index.js:12 2
index.js:13 6
index.js:16 ---- Async Wasm Module
index.js:17 ƒ $_Z4facti() { [native code] }
index.js:18 1
index.js:19 2
index.js:20 6
```
