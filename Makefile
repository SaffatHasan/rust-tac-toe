run-cli:
	cargo run --bin rust-tac-toe-cli

run-gui:
	cargo run --bin rust-tac-toe-gui

run-wasm: crates/wasm/pkg
	@cd crates/wasm && python -m http.server 8000 --bind 127.0.0.1

crates/wasm/pkg: crates/wasm/Cargo.toml crates/wasm/src/lib.rs crates/engine
	wasm-pack build crates/wasm --target web --release

build:
	cargo build

test:
	cargo test

