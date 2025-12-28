// Console interface for the tic-tac-toe game.

use rust_tac_toe_engine::{GameEngine, GameStatus, InvalidGameMoveError, Player, Position};
use std::io::{self, Write};

pub fn run() {
    let mut engine = GameEngine::new();

    loop {
        println!("{}", board_as_string(&engine));
        match engine.status {
            GameStatus::Win(player) => {
                println!("Game over! Winner: {:?}", player);
                if !start_new_game(&mut engine) {
                    break;
                }
            }
            GameStatus::Draw => {
                println!("Game over! It's a draw!");
                if !start_new_game(&mut engine) {
                    break;
                }
            }
            GameStatus::Ongoing => {
                println!("Current player: {:?}", engine.current_player);
                game_loop(&mut engine);
            }
        }
    }
}

/// Prompt the user to start a new game. Returns true if a new game was started.
pub fn start_new_game(engine: &mut GameEngine) -> bool {
    print!("Press 'r' to reset the game or any other key to exit: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() != "r" {
        return false;
    }
    engine.reset();
    return true;
}

pub fn board_as_string(engine: &GameEngine) -> String {
    let mut board_str: String = String::new();
    for i in (0..9).step_by(3) {
        // Map the Player enum to a string for display
        let get_char = |idx: usize| match engine.board[idx] {
            Some(Player::X) => "X",
            Some(Player::O) => "O",
            None => " ",
        };

        // Print the row with vertical dividers
        board_str.push_str(&format!(
            " {} | {} | {} \n",
            get_char(i),
            get_char(i + 1),
            get_char(i + 2)
        ));

        // Print horizontal divider between rows (but not after the last row)
        if i < 6 {
            board_str.push_str("-----------\n");
        }
    }
    return board_str;
}

pub fn game_loop(engine: &mut GameEngine) {
    print!("Enter your move (0-8) or 'r' to reset: ");
    io::stdout().flush().unwrap();

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if handle_input(engine, input) {
            break;
        }
    }
}

/// Handle user input for making a move or resetting the game.
/// Returns true if the input was handled successfully.
pub fn handle_input(engine: &mut GameEngine, input: &str) -> bool {
    if input == "r" {
        engine.reset();
        return true;
    }

    let Ok(pos) = input.parse::<u8>() else {
        println!("Invalid input. Please enter a number between 0 and 8 or 'r' to reset.");
        return false;
    };

    let Some(pos) = Position::new(pos) else {
        println!("Invalid position. Please enter a number between 0 and 8.");
        return false;
    };

    if let Err(e) = engine.validate_move(pos) {
        match e {
            InvalidGameMoveError::GameAlreadyWon => {
                println!("Error: The game is already won! Press 'r' to reset.");
            }
            InvalidGameMoveError::SpaceOccupied => {
                println!("Error: That space is already occupied. Try another.");
            }
        }
        return false;
    }

    if let Err(e) = engine.play_move(pos) {
        println!("Error playing move: {:?}", e);
        return false;
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_tac_toe_engine::{GameEngine, Position};

    #[test]
    fn test_print_board() {
        let mut engine = GameEngine::new();

        for &p in [0, 1, 3, 4, 6].iter() {
            engine.play_move(Position::new(p).unwrap()).unwrap();
        }

        let board_str = board_as_string(&engine);
        let expected_str = " X | O |   \n-----------\n X | O |   \n-----------\n X |   |   \n";
        assert_eq!(board_str, expected_str);
    }

    #[test]
    fn test_handle_input_reset() {
        let mut engine = GameEngine::new();
        engine.play_move(Position::new(0).unwrap()).unwrap();
        handle_input(&mut engine, "r");
        assert_eq!(engine.board, [None; 9]);
        assert_eq!(engine.current_player, Player::X);
        assert_eq!(engine.status, GameStatus::Ongoing);
    }
}
