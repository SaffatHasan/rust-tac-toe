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
        print!("Enter your move (0-8) or 'r' to reset: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "r" {
            match engine.handle_event(GameEvent::Reset) {
                Ok(()) => {
                    continue;
                }
                Err(_) => {
                    // Reset should never fail, but handle it just in case
                    eprintln!("Error: Failed to reset game.");
                    break;
                }
            }
        }

        match input.parse::<u8>() {
            Ok(pos) => {
                if let Some(position) = Position::new(pos) {
                    match engine.handle_event(GameEvent::PlayMove(position)) {
                        Ok(()) => {
                            // Move was successful
                        }
                        Err(EventError::GameAlreadyWon) => {
                            println!("Error: The game is already won! Press 'r' to reset.");
                        }
                        Err(EventError::SpaceOccupied) => {
                            println!("Error: That space is already occupied. Try another.");
                        }
                    }
                } else {
                    println!("Invalid position. Please enter a number between 0 and 8.");
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 0 and 8 or 'r' to reset.");
            }
        }
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
