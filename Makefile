run-cli:
	cargo run --bin rust-tac-toe-cli

run-gui-exe:
	cargo run --bin rust-tac-toe-gui-exe

run-gui-wasm:
	@cd crates/gui-wasm && trunk serve --open

run-wasm:
	@cd crates/wasm && trunk serve --open

build:
	cargo build

test:
	cargo test

