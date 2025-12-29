# 🎮 Rust-tac-toe

Learning project implementing `Tic-Tac-Toe` in Rust with multiple interface options.

## Features

- **Console Interface**: Play in the terminal
- **GUI Interface**: Desktop GUI using eframe ([Try me](https://saffathasan.github.io/rust-tac-toe/gui-wasm))
- **WebAssembly**: Play in the browser ([Try me](https://saffathasan.github.io/rust-tac-toe/wasm))

## Building and Running

See the [Makefile](Makefile) for building and running commands.

Requires [rust](https://rust-lang.org/tools/install/) and [trunk](https://trunkrs.dev/#install).

## Layout

```text
.
├── crates
│   ├── cli          # Handles CLI
│   ├── engine       # Core Tic-Tac-Toe logic
│   ├── gui-core     # Shared eframe GUI for exe and wasm
│   ├── gui-exe      # Desktop native target
│   ├── gui-wasm     # Wasm target -- eframe for UI
│   └── wasm         # engine as WASM -- native HTML/JS for UI
├── Makefile         # Shortcuts for common tasks (e.g. `make run-wasm`)
└── Cargo.toml       # Workspace configuration
```

## Screenshots

| CLI                 | GUI                 | Web (WASM)           |
| ------------------- | ------------------- | -------------------- |
| ![](assets/cli.png) | ![](assets/gui.png) | ![](assets/wasm.png) |

## Size Benchmarks (wasm)

The wasm (UI is done with native HTML/js) is very small.

- Standard: 22kB (9.9kB gzipped)
- `wee_alloc`: 15.7kB (7.6kB gzipped)
- `serde_wasm_bindgen`: 19.7kB (10.2kB gzipped)

23% reduction in transfer size with `wee_alloc`, but introducing `serde_wasm_bindgen`
causes it to run faster (no need to use JSON.Parse) but has a larger binary size.

However, it seems `wee_alloc` is no longer maintained so it shouldn't be used for real
production applications.

## Size Benchmarks (gui-wasm)

Eframe is huge comparively speaking. The resulting wasm is 2,263 kB (1,130 kB gzipped)
which is nearly a 10x increase in size.
