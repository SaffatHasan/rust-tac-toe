#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn next(&self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[cfg_attr(feature = "wasm", serde(tag = "type", content = "value"))]
pub enum GameStatus {
    Win { player: Player, line: [usize; 3] },
    Draw,
    Ongoing,
}

/// Represents errors that can occur when handling game events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InvalidGameMoveError {
    /// Attempted to play a move when the game is already won.
    GameAlreadyWon,
    /// Attempted to play a move on an already occupied space.
    SpaceOccupied,
}

/// A valid board position for tic-tac-toe, guaranteed to be in the range 0-8.
///
/// This type makes it impossible to construct an invalid position,
/// ensuring that only valid board indices can be used.
#[derive(Copy, Clone)]
pub struct Position(u8);

impl Position {
    pub fn new(pos: u8) -> Option<Self> {
        if pos > 8 {
            return None;
        }
        Some(Self(pos))
    }

    // Convert the Position to a usize index for array access.
    pub fn to_index(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize))]
#[serde(rename_all = "camelCase")]
pub struct GameEngine {
    pub board: [Option<Player>; 9],
    pub current_player: Player,
    pub status: GameStatus,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            current_player: Player::X,
            status: GameStatus::Ongoing,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn play_move(&mut self, pos: Position) -> Result<(), InvalidGameMoveError> {
        self.validate_move(pos)?;

        // Apply Move
        self.board[pos.to_index()] = Some(self.current_player);

        // Update Game Status
        self.status = self.calculate_status();

        // Switch Player
        self.current_player = self.current_player.next();
        Ok(())
    }

    pub fn validate_move(&self, pos: Position) -> Result<(), InvalidGameMoveError> {
        if self.status != GameStatus::Ongoing {
            return Err(InvalidGameMoveError::GameAlreadyWon);
        }
        let pos: usize = pos.to_index();
        if self.board[pos].is_some() {
            return Err(InvalidGameMoveError::SpaceOccupied);
        }
        Ok(())
    }

    pub fn calculate_status(&self) -> GameStatus {
        const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
            // horizontal
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            // vertical
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            // diagonal
            [0, 4, 8],
            [2, 4, 6],
        ];

        for [a, b, c] in WINNING_COMBINATIONS.iter() {
            if let (Some(p1), Some(p2), Some(p3)) = (self.board[*a], self.board[*b], self.board[*c])
            {
                if p1 == p2 && p2 == p3 {
                    {
                        return GameStatus::Win {
                            player: p1,
                            line: [*a, *b, *c],
                        };
                    }
                }
            }
        }

        // Check draw (if no winner)
        if self.board.iter().all(|&p| p.is_some()) {
            return GameStatus::Draw;
        }

        return GameStatus::Ongoing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_position_new() {
        assert!(Position::new(0).is_some());
        assert!(Position::new(8).is_some());
        assert!(Position::new(9).is_none());
        assert!(Position::new(255).is_none());
    }

    #[test]
    fn test_update_next_player() {
        assert_eq!(Player::O.next(), Player::X);
        assert_eq!(Player::X.next(), Player::O);
    }

    #[test]
    fn test_play_move() {
        let mut engine = GameEngine::new();
        let pos = Position::new(0).unwrap();
        let _ = engine.play_move(pos);
        assert_eq!(engine.board[0], Some(Player::X));
        assert_eq!(engine.current_player, Player::O);
    }

    #[test]
    fn test_winner_detection() {
        let mut engine = GameEngine::new();
        let moves = [0, 3, 1, 4, 2]; // X wins
        for &m in &moves {
            let pos = Position::new(m).unwrap();
            let _ = engine.play_move(pos);
        }
        assert_eq!(
            engine.status,
            GameStatus::Win {
                player: Player::X,
                line: [0, 1, 2]
            }
        );
    }

    #[test]
    fn test_draw_detection() {
        let mut engine = GameEngine::new();
        let moves = [0, 1, 2, 4, 3, 5, 7, 6, 8]; // Draw
        for &m in &moves {
            let pos = Position::new(m).unwrap();
            let _ = engine.play_move(pos);
        }
        assert_eq!(engine.status, GameStatus::Draw);
    }

    #[test]
    fn test_winner_detection_full_board() {
        let mut engine: GameEngine = GameEngine::new();
        let moves = [0, 1, 2, 4, 3, 5, 7, 8, 6]; // X wins and fills the board
        for &m in &moves {
            let pos = Position::new(m).unwrap();
            let _ = engine.play_move(pos);
        }
        assert_eq!(
            engine.status,
            GameStatus::Win {
                player: Player::X,
                line: [0, 3, 6]
            }
        );
    }

    #[test]
    fn test_reset() {
        let mut engine = GameEngine::new();
        let pos = Position::new(0).unwrap();
        let _ = engine.play_move(pos);
        engine.reset();
        assert_eq!(engine.board, [None; 9]);
        assert_eq!(engine.current_player, Player::X);
        assert_eq!(engine.status, GameStatus::Ongoing);
    }

    #[test]
    fn test_serialize_game_status() {
        let mut engine = GameEngine::new();
        engine.status = GameStatus::Win {
            player: Player::O,
            line: [0, 1, 2],
        };
        let win = serde_json::to_value(&engine).unwrap();
        let expected = serde_json::json!({
            "board": [null, null, null, null, null, null, null, null, null],
            "currentPlayer": "X",
            "status": {
                "type": "Win",
                "value": {
                    "player": "O",
                    "line": [0, 1, 2],
                },
            }
        });

        assert_eq!(win, expected,);
    }
}
