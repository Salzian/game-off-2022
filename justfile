_default:
    @just --list

run-dev:
    cargo run

build-web:
    rustup target add wasm32-unknown-unknown
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --out-dir dist/bin --target web target/wasm32-unknown-unknown/release/game-off-2022.wasm

_assert-is-installed tool:
    #!/usr/bin/env bash
    set -euxo pipefail # https://github.com/casey/just#safer-bash-shebang-recipes

    if ! {{tool}} --version &> /dev/null; then
        echo "{{tool}} is not installed!";
        exit 1;
    fi
