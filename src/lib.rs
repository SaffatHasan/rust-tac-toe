mod engine;

// Export engine types and functions
pub use engine::{EventError, GameEngine, GameEvent, GameStatus, Player, Position};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

/// WASM-friendly wrapper for the game engine
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct WasmGameEngine {
    engine: GameEngine,
}

#[cfg(feature = "wasm")]
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

    /// Get the board state as a JSON string
    /// Board layout: [0, 1, 2, 3, 4, 5, 6, 7, 8]
    pub fn get_board(&self) -> String {
        let board: Vec<String> = self
            .engine
            .board
            .iter()
            .map(|p| match p {
                Player::X => "X".to_string(),
                Player::O => "O".to_string(),
                Player::None => "".to_string(),
            })
            .collect();

        serde_json::to_string(&board).unwrap_or_else(|_| "[]".to_string())
    }

    /// Get the current player ("X" or "O")
    pub fn get_current_player(&self) -> String {
        match self.engine.current_player {
            Player::X => "X".to_string(),
            Player::O => "O".to_string(),
            Player::None => "None".to_string(),
        }
    }

    /// Get the game status
    /// Returns: "Ongoing", "Draw", or "WinX" / "WinO"
    pub fn get_status(&self) -> String {
        match self.engine.status {
            GameStatus::Ongoing => "Ongoing".to_string(),
            GameStatus::Draw => "Draw".to_string(),
            GameStatus::Win(Player::X) => "WinX".to_string(),
            GameStatus::Win(Player::O) => "WinO".to_string(),
            GameStatus::Win(Player::None) => "None".to_string(),
        }
    }

    /// Get the complete game state as JSON
    pub fn get_state(&self) -> String {
        let board: Vec<String> = self
            .engine
            .board
            .iter()
            .map(|p| match p {
                Player::X => "X".to_string(),
                Player::O => "O".to_string(),
                Player::None => "".to_string(),
            })
            .collect();

        let current_player = match self.engine.current_player {
            Player::X => "X",
            Player::O => "O",
            Player::None => "None",
        };

        let status = match self.engine.status {
            GameStatus::Ongoing => "Ongoing",
            GameStatus::Draw => "Draw",
            GameStatus::Win(Player::X) => "WinX",
            GameStatus::Win(Player::O) => "WinO",
            GameStatus::Win(Player::None) => "None",
        };

        let state = serde_json::json!({
            "board": board,
            "currentPlayer": current_player,
            "status": status,
        });

        state.to_string()
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

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn init() {
    // Initialize WASM module
}
