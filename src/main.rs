mod engine;

// #![cfg_attr(target_arch = "wasm32", allow(dead_code))]
#[cfg_attr(target_arch = "wasm32", allow())]
// Console
#[cfg(feature = "console")]
mod console;
#[cfg(feature = "console")]
use crate::console::run;
#[cfg(feature = "console")]
fn main() {
    run();
}

#[cfg(feature = "gui")]
mod gui;
#[cfg(feature = "gui")]
fn main() {
    crate::gui::run_desktop();
}
