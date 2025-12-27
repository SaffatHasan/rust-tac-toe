// Save wasm space by using a smaller allocator
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Export engine types and functions
pub use rust_tac_toe_engine::{EventError, GameEngine, GameEvent, GameStatus, Player, Position};

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
    pub fn play_move(&mut self, position: u8) -> Result<(), String> {
        let pos = Position::new(position).ok_or("Invalid position (must be 0-8)".to_string())?;

        self.engine
            .handle_event(GameEvent::PlayMove(pos))
            .map_err(|e| format!("{:?}", e))
    }

    /// Reset the game
    pub fn reset(&mut self) -> Result<(), String> {
        self.engine
            .handle_event(GameEvent::Reset)
            .map_err(|e| format!("{:?}", e))
    }

    /// Get the complete game state as JSON
    pub fn get_state(&self) -> String {
        return serde_json::to_string(&self.engine).unwrap_or_else(|_| "{}".to_string());
    }

    /// Check if a move is valid at the given position
    pub fn is_valid_move(&self, position: u8) -> bool {
        let pos = match Position::new(position) {
            Some(p) => p,
            None => return false,
        };
        self.engine.validate_event(GameEvent::PlayMove(pos)).is_ok()
    }
}

#[wasm_bindgen(start)]
pub fn init() {
    // Initialize WASM module
}

// tests
#[cfg(test)]
mod tests {
    use super::*;
    use rust_tac_toe_engine::{GameEngine, GameEvent, Position};

    #[test]
    fn test_get_state() {
        let mut engine = GameEngine::new();

        for &p in [0, 1, 3, 4, 6].iter() {
            engine
                .handle_event(GameEvent::PlayMove(Position::new(p).unwrap()))
                .unwrap();
        }

        let wasm_engine = WasmGameEngine { engine };

        let state_json = wasm_engine.get_state();
        let expected_json =
            r#"{"board":["X","O","","X","O","","X","",""],"currentPlayer":"O","status":"WinX"}"#;

        assert_eq!(state_json, expected_json);
    }
}
