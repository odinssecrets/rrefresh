all:
	wasm-pack build --target=no-modules
	rm -f -r wasm/ && mkdir wasm
	cp pkg/rrefresh_bg.wasm wasm/
	cp pkg/rrefresh.js wasm/
