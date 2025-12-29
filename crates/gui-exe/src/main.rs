use eframe::egui;
use rust_tac_toe_gui_core::TicTacToeApp;

fn main() {
    let mut native_options = eframe::NativeOptions::default();

    // Configure the viewport for a stable, polished window experience
    native_options.viewport = egui::ViewportBuilder::default()
        .with_inner_size([400.0, 580.0]) // Slightly larger to account for padding
        // .with_min_inner_size([400.0, 580.0]) // Prevents UI breakage from shrinking
        .with_resizable(false)
        .with_maximize_button(false)
        .with_drag_and_drop(false);

    let _ = eframe::run_native(
        "Rust Tic-Tac-Toe",
        native_options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}
