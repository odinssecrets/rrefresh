# RRefresh Rust/WASM Extentsion for tab refreshing

This is a web extension that is used for auto-refreshing tabs.

This project was started to learn rust and webasm and to create
a usable and customizable refreshing extension.

This project is still in a beta state and is being developed.

## System Requirements
Only tested/developed for x86_64 Linux
- make
- rustc (tested on 1.60.0-nightly)
- cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Build 

### Setup steps
- Install rustup
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Select the nightly version of rustc
    `rustup default nightly`
- Install wasm-pack
    `cargo install wasm-pack`

### Build instructions
- Run `make` to build the release version or `make debug` to build a dev version.
- Run `make package` to create the zipped package version of the extension.

### Local testing
- Testing via web-ext:
    - npm install --global web-ext
    - `make debug`
    - Do `web-ext run` in the top level directory
- Testing via local browser
    - `make debug`
    - Follow the instructions [here](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Your_first_WebExtension#installing)
