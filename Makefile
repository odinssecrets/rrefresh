all: rust files

release: rust-release files

files:
	rm -f -r wasm/ && mkdir wasm
	cp pkg/rrefresh_bg.wasm wasm/
	cp pkg/rrefresh.js wasm/

rust:
	wasm-pack build --dev --target=no-modules

rust-release:
	wasm-pack build --release --target=no-modules
