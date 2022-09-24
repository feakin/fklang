wasm-pack build  --scope feakin --target=browser --out-name fkl-wasm-web --out-dir pkg-web
sed -i.old 's/"name": "@feakin\/fkl-wasm"/"name": "@feakin\/fkl-wasm-web"/' pkg-web/package.json
#sed -i.old 's/"name": "diamond-wasm"/"name": "diamond-types-web"/' pkg-web/package.json

wasm-pack build  --scope feakin --target=nodejs --out-name fkl-wasm-node --out-dir pkg-node
sed -i.old '' 's/"name": "@feakin/fkl-wasm"/"name": "@feakin/fkl-wasm-node"/' pkg-node/package.json
