mod console;
mod engine;

use crate::console::run;
use crate::engine::GameEngine;

fn main() {
    println!("Hello, world!");

    run(GameEngine::new());
}
