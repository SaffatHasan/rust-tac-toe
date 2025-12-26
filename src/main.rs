mod console;
mod engine;

#[cfg(feature = "gui")]
mod gui;

use crate::console::run;
use crate::engine::GameEngine;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let use_gui = args.iter().any(|a| a == "--gui" || a == "-g");

    if use_gui {
        #[cfg(feature = "gui")]
        {
            crate::gui::run_desktop();
            return;
        }

        #[cfg(not(feature = "gui"))]
        {
            eprintln!(
                "GUI not enabled in this build — compile with `--features gui` to enable. Running console instead."
            );
        }
    }

    run(GameEngine::new());
}
