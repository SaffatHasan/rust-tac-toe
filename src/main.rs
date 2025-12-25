mod console;
mod engine;

use crate::console::run;
use crate::engine::{GameEngine, GameEvent, Position};

fn main() {
    println!("Hello, world!");

    let mut engine = GameEngine::new();
    engine.handle_event(GameEvent::PlayMove(Position::new(0).unwrap()));
    engine.handle_event(GameEvent::Reset);
    run(engine);
}
