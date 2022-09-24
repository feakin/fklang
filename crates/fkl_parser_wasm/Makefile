all: setup build

setup:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

build: build-node build-web
	rm pkg-*/package.json.old

PKG_NAME=@feakin\/fkl-wasm

build-web:
	wasm-pack build  --scope feakin --target=browser --out-name fkl-wasm-web --out-dir pkg-web
	sed -i.old 's/"name": "${PKG_NAME}"/"name": "${PKG_NAME}-web"/' pkg-web/package.json

build-node:
	wasm-pack build  --scope feakin --target=nodejs --out-name fkl-wasm-node --out-dir pkg-node
	sed -i.old 's/"name": "${PKG_NAME}"/"name": "${PKG_NAME}-node"/' pkg-node/package.json

publish: publish-web publish-node

publish-web:
	cd pkg-web && npm publish --access=public

publish-node:
	cd pkg-node && npm publish --access=public
