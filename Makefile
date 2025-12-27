run-console:
	cargo run --bin rust-tac-toe-cli

run-gui:
	cargo run --bin rust-tac-toe-gui

run-wasm: build-wasm
	python -m http.server 8000

build:
	cargo build

build-wasm:
	wasm-pack build crates/wasm --target web --release

test:
	cargo test

