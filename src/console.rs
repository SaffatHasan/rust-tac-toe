// Console interface for the tic-tac-toe game.

use crate::engine::{EventError, GameEngine, GameEvent, Position};
use std::io::{self, Write};

pub fn run(engine: GameEngine) {
    let mut engine = engine;

    loop {
        print_board(&engine);
        if engine.winner != crate::engine::Player::None {
            println!("Game over! Winner: {:?}", engine.winner);
            print!("Press 'r' to reset the game or any other key to exit: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim() == "r" {
                let _ = engine.handle_event(GameEvent::Reset);
                continue;
            } else {
                break;
            }
        }
        println!("Current player: {:?}", engine.current_player);
        // Let all validation happen inside get_input
        let event: GameEvent = get_input(&engine);
        let _ = engine.handle_event(event);
    }
}

pub fn print_board(engine: &GameEngine) {
    for i in 0..9 {
        let symbol = match engine.board[i] {
            crate::engine::Player::X => "X",
            crate::engine::Player::O => "O",
            crate::engine::Player::None => ".",
        };
        print!(" {} ", symbol);
        if i % 3 == 2 {
            println!();
        }
    }
}

pub fn get_input(engine: &GameEngine) -> GameEvent {
    loop {
        print!("Enter your move (0-8) or 'r' to reset: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "r" {
            return GameEvent::Reset;
        }

        let Ok(pos) = input.parse::<u8>() else {
            println!("Invalid input. Please enter a number between 0 and 8 or 'r' to reset.");
            continue;
        };

        let Some(position) = Position::new(pos) else {
            println!("Invalid position. Please enter a number between 0 and 8.");
            continue;
        };

        let move_event: GameEvent = GameEvent::PlayMove(position);

        match engine.validate_event(move_event) {
            Ok(()) => {
                return move_event;
            }
            Err(error) => {
                match error {
                    EventError::GameAlreadyWon => {
                        println!("Error: The game is already won! Press 'r' to reset.");
                    }
                    EventError::SpaceOccupied => {
                        println!("Error: That space is already occupied. Try another.");
                    }
                }
                continue;
            }
        }
    }
}
