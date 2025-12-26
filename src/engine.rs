#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
    None,
}

/// Represents errors that can occur when handling game events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventError {
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
pub enum GameEvent {
    PlayMove(Position),
    Reset,
}

pub struct GameEngine {
    pub board: [Player; 9],
    pub current_player: Player,
    pub winner: Player,
}

impl GameEngine {
    pub fn new() -> Self {
        Self {
            board: [Player::None; 9],
            current_player: Player::X,
            winner: Player::None,
        }
    }

    pub fn handle_event(&mut self, event: GameEvent) -> Result<(), EventError> {
        let is_event_valid: Result<(), EventError> = self.validate_event(event);
        if is_event_valid.is_err() {
            return is_event_valid;
        }

        match event {
            GameEvent::PlayMove(pos) => {
                let pos: usize = pos.to_index();
                self.board[pos] = self.current_player;
                self.update_winner();
                self.update_next_player();
                Ok(())
            }
            GameEvent::Reset => {
                *self = Self::new();
                Ok(())
            }
        }
    }

    pub fn validate_event(&self, event: GameEvent) -> Result<(), EventError> {
        match event {
            GameEvent::PlayMove(pos) => {
                if self.winner != Player::None {
                    return Err(EventError::GameAlreadyWon);
                }
                let pos: usize = pos.to_index();
                if self.board[pos] != Player::None {
                    return Err(EventError::SpaceOccupied);
                }
                Ok(())
            }
            GameEvent::Reset => Ok(()),
        }
    }

    pub fn update_next_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::None => Player::None,
        };
    }

    pub fn update_winner(&mut self) {
        let winning_combinations: [[usize; 3]; 8] = [
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

        for combo in winning_combinations.iter() {
            if self.board[combo[0]] != Player::None
                && self.board[combo[0]] == self.board[combo[1]]
                && self.board[combo[1]] == self.board[combo[2]]
            {
                self.winner = self.board[combo[0]];
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_new() {
        assert!(Position::new(0).is_some());
        assert!(Position::new(8).is_some());
        assert!(Position::new(9).is_none());
        assert!(Position::new(255).is_none());
    }

    #[test]
    fn test_update_next_player() {
        let mut engine = GameEngine::new();
        assert_eq!(engine.current_player, Player::X);
        engine.update_next_player();
        assert_eq!(engine.current_player, Player::O);
        engine.update_next_player();
        assert_eq!(engine.current_player, Player::X);
    }

    #[test]
    fn test_update_next_player_none() {
        let mut engine = GameEngine::new();
        engine.current_player = Player::None;
        engine.update_next_player();
        assert_eq!(engine.current_player, Player::None);
    }

    #[test]
    fn test_play_move() {
        let mut engine = GameEngine::new();
        let pos = Position::new(0).unwrap();
        let _ = engine.handle_event(GameEvent::PlayMove(pos));
        assert_eq!(engine.board[0], Player::X);
        assert_eq!(engine.current_player, Player::O);
    }

    #[test]
    fn test_winner_detection() {
        let mut engine = GameEngine::new();
        let moves = [0, 3, 1, 4, 2]; // X wins
        for &m in &moves {
            let pos = Position::new(m).unwrap();
            let _ = engine.handle_event(GameEvent::PlayMove(pos));
        }
        assert_eq!(engine.winner, Player::X);
    }

    #[test]
    fn test_reset() {
        let mut engine = GameEngine::new();
        let pos = Position::new(0).unwrap();
        let _ = engine.handle_event(GameEvent::PlayMove(pos));
        let _ = engine.handle_event(GameEvent::Reset);
        assert_eq!(engine.board, [Player::None; 9]);
        assert_eq!(engine.current_player, Player::X);
        assert_eq!(engine.winner, Player::None);
    }
}
