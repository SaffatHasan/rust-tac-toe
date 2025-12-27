# rust-tac-toe

Learning project implementing Tic-Tac-Toe in Rust with multiple interface options.

## Features

- **Console Interface**: Play in the terminal
- **GUI Interface**: Desktop GUI using eframe
- **WebAssembly**: Play in the browser!

## Building and Running

### Console Version

```bash
cargo run
```

### GUI Version

```bash
cargo run --features gui
```

### WebAssembly Version

#### Requirements

- `wasm-pack`: Install with `cargo install wasm-pack`

#### Build

```bash
./wasm-build.sh
```

Or manually:

```bash
wasm-pack build --target bundler --release -- --features wasm
```
