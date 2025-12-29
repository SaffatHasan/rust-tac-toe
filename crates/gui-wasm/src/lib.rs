#[cfg(target_arch = "wasm32")]
use rust_tac_toe_gui_core::TicTacToeApp;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)] // This makes it run automatically on load
pub fn start_app() {
    // Redirect panics to the console
    console_error_panic_hook::set_once();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id",
                web_options,
                Box::new(|_cc| Box::new(TicTacToeApp::default())),
            )
            .await
            .expect("failed to start eframe");
    });
}
