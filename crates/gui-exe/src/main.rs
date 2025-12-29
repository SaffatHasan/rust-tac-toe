use eframe::egui;
use rust_tac_toe_gui_core::TicTacToeApp;
fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default().with_inner_size([380.0, 500.0]);
    let _ = eframe::run_native(
        "Rust Tic-Tac-Toe",
        native_options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}
