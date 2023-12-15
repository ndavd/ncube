CC=cargo

default: native

all: native web

run: FORCE
	$(CC) r --release

native: FORCE
	$(CC) b --release

web: FORCE
	$(CC) b \
		--profile tiny \
		--target wasm32-unknown-unknown
	wasm-bindgen \
		--out-name ncube \
		--out-dir	web/wasm \
		--target web \
		target/wasm32-unknown-unknown/tiny/ncube.wasm

clean: FORCE
	-rm -r target
	-rm web/public/ncube.zip

FORCE:
