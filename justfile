default:
    @just --list

run-dev:
    cargo run

build:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir dist/bin --target web target/wasm32-unknown-unknown/release/game-off-2022.wasm
