CC=cargo b

default: native

all: native web

native: FORCE
	$(CC) --release

web: FORCE
	$(CC) \
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
