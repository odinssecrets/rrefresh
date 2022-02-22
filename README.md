# RRefresh Rust/WASM Extentsion for tab refreshing

This is a web extension that is used for auto-refreshing tabs.

## System Requirements
- Only tested/developed for x86_64 Linux
- make
- rustc (tested on 1.60.0-nightly)
- cargo
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Build 

#Setup steps
- Install rustup
    `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Select the nightly version of rustc
    `rustup default nightly`
- Install wasm-pack
    `cargo install wasm-pack`

# Build instructions
- Run `make` to build the release version or `make debug` to build a dev version.
- Run `make package` to create the zipped package version of the extension.
