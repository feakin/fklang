all: clean build publish

setup:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

clean:
	rm -rf pkg-*

build: build-node build-web
	rm pkg-*/package.json.old

PKG_NAME=@feakin\/fkl-wasm

build-web:
	wasm-pack build --target web --out-dir pkg-web --out-name fkl .
	sed -i.old 's/"name": "fkl-wasm"/"name": "${PKG_NAME}-web"/' pkg-web/package.json

build-node:
	wasm-pack build --target nodejs --out-dir pkg-node --out-name fkl .
	sed -i.old 's/"name": "fkl-wasm"/"name": "${PKG_NAME}-node"/' pkg-node/package.json

publish: publish-web publish-node

publish-web:
	cd pkg-web && npm publish --access=public

publish-node:
	cd pkg-node && npm publish --access=public
