all: rust-release files

debug: rust-debug files

files:
	rm -f -r wasm/ && mkdir wasm
	cp pkg/rrefresh_bg.wasm wasm/
	cp pkg/rrefresh.js wasm/

rust-debug:
	wasm-pack build --dev --target=no-modules

rust-release:
	wasm-pack build --release --target=no-modules
