set -e

RUSTFLAGS=""

rm -rf pkg-*

wasm-pack build --target web --out-dir pkg-web --out-name fkl .
sed -i.old 's/"name": "fkl-wasm"/"name": "@feakin\/fkl-wasm-web"/' pkg-web/package.json

wasm-pack build --target nodejs --out-dir pkg-node --out-name fkl .
sed -i.old 's/"name": "fkl-wasm"/"name": "@feakin/fkl-wasm-node"/' pkg-node/package.json

brotli -f pkg-web/*.wasm
