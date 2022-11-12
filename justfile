_default:
    @just --list

run-dev *FLAGS:
    @just _assert-is-installed cargo
    cargo run {{FLAGS}}

build-web:
    @just _assert-is-installed rustup
    @just _assert-is-installed cargo
    @just _assert-is-installed wasm-bindgen

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
