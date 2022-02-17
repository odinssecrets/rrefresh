all: rust-release files

debug: rust-debug files

package: all
	rm -f rrefresh.zip
	zip -r rrefresh.zip wasm/
	zip -r rrefresh.zip js/
	zip -r rrefresh.zip html/*.html
	zip -r rrefresh.zip html/*.js
	zip -r rrefresh.zip html/*.css
	zip -r rrefresh.zip icon/
	zip -r rrefresh.zip manifest.json

files:
	rm -f -r wasm/ && mkdir wasm
	cp pkg/rrefresh_bg.wasm wasm/
	cp pkg/rrefresh.js wasm/

rust-debug:
	wasm-pack build --dev --target=no-modules

rust-release:
	wasm-pack build --release --target=no-modules

clean:
	rm -r pkg/ wasm/ target/
