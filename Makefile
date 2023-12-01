CC=cargo b

default: FORCE
	$(CC) --release

web: FORCE
	$(CC) \
		--profile tiny \
		--target wasm32-unknown-unknown
	wasm-bindgen \
		--out-name ncube \
		--out-dir	wasm \
		--target web \
		target/wasm32-unknown-unknown/tiny/ncube.wasm
	zip \
		-j \
		web/public/ncube.zip \
		wasm/*
	rm -r wasm

clean: FORCE
	-rm -r target
	-rm web/public/ncube.zip

FORCE:
