// Save wasm space by using a smaller allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Export engine types and functions
pub use rust_tac_toe_engine::{GameEngine, Position};

use wasm_bindgen::prelude::*;

/// WASM-friendly wrapper for the game engine
#[wasm_bindgen]
pub struct WasmGameEngine {
    engine: GameEngine,
}

#[wasm_bindgen]
impl WasmGameEngine {
    /// Create a new game
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmGameEngine {
        WasmGameEngine {
            engine: GameEngine::new(),
        }
    }

    /// Play a move at the given position (0-8)
    pub fn play_move(&mut self, position: u8) -> Result<(), JsError> {
        let pos = Position::new(position).ok_or(JsError::new("Invalid position (must be 0-8)"))?;

        self.engine
            .play_move(pos)
            .map_err(|e| JsError::new(&format!("{:?}", e)))
    }

    /// Reset the game
    pub fn reset(&mut self) {
        self.engine.reset();
    }

    /// Get the complete game state as JSON
    pub fn get_state(&self) -> Result<JsValue, JsError> {
        return serde_wasm_bindgen::to_value(&self.engine)
            .map_err(|e| JsError::new(&format!("Serialization error: {}", e)));
    }

    /// Check if a move is valid at the given position
    pub fn is_valid_move(&self, position: u8) -> bool {
        let pos = match Position::new(position) {
            Some(p) => p,
            None => return false,
        };
        self.engine.validate_move(pos).is_ok()
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Initialize WASM module
}
