#[cfg(not(feature = "gui"))]
mod console;

mod engine;

#[cfg(feature = "gui")]
mod gui;

#[cfg(not(feature = "gui"))]
use crate::console::run;

#[cfg(feature = "gui")]
fn main() {
    crate::gui::run_desktop();
}

#[cfg(not(feature = "gui"))]
fn main() {
    run();
}
