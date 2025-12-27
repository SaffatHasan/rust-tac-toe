run:
	cargo run --features console

run-gui:
	cargo run --features gui

run-wasm: build-wasm
	python -m http.server 8000

build:
	cargo build

build-wasm:
	wasm-pack build --target web --release -- --features wasm

test:
	cargo test

