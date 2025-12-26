//! eframe/egui GUI for Tic-Tac-Toe with full game logic integration.
//!
//! Provides a desktop GUI using the GameEngine from engine.rs.
//! For wasm target later, enable eframe's `wasm` feature and use the web start API.

use crate::engine::{GameEngine, GameEvent, GameStatus, Player, Position};
use eframe::{egui, App};

// Layout constants
const MIN_CELL: f32 = 40.0;
const MAX_CELL: f32 = 200.0;
const RESERVED_VERTICAL: f32 = 120.0; // estimated height used by headings/controls

/// The main eframe application for tic-tac-toe.
pub struct TicTacToeApp {
    engine: GameEngine,
}

impl Default for TicTacToeApp {
    fn default() -> Self {
        Self {
            engine: GameEngine::new(),
        }
    }
}

impl App for TicTacToeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🎮 Tic-Tac-Toe");
            ui.separator();

            self.render_status(ui);

            ui.separator();

            let cell_size = compute_cell_size(ui.available_size());
            self.render_board(ui, cell_size);

            ui.separator();

            if ui
                .button(egui::RichText::new("🔄 Reset Game").size(16.0))
                .clicked()
            {
                let _ = self.engine.handle_event(GameEvent::Reset);
            }
        });
    }
}

impl TicTacToeApp {
    fn render_status(&self, ui: &mut egui::Ui) {
        match self.engine.status {
            GameStatus::Ongoing => {
                let player_text = match self.engine.current_player {
                    Player::X => "❌ X's Turn",
                    Player::O => "⭕ O's Turn",
                    Player::None => "Error",
                };
                ui.label(egui::RichText::new(player_text).size(18.0).strong());
            }
            GameStatus::Win(winner) => {
                let winner_text = match winner {
                    Player::X => "🎉 X Wins!",
                    Player::O => "🎉 O Wins!",
                    Player::None => "Error",
                };
                ui.label(
                    egui::RichText::new(winner_text)
                        .size(20.0)
                        .strong()
                        .color(egui::Color32::GREEN),
                );
            }
            GameStatus::Draw => {
                ui.label(
                    egui::RichText::new("🤝 It's a Draw!")
                        .size(20.0)
                        .strong()
                        .color(egui::Color32::YELLOW),
                );
            }
        }
    }

    fn render_board(&mut self, ui: &mut egui::Ui, cell_size: f32) {
        // If game won, compute winning line to highlight
        let winning = if let GameStatus::Win(_) = self.engine.status {
            winning_line(&self.engine.board)
        } else {
            None
        };

        for row in 0..3 {
            ui.horizontal(|ui| {
                for col in 0..3 {
                    let idx = row * 3 + col;
                    let cell = self.engine.board[idx];

                    let (symbol, color) = match cell {
                        Player::X => ("X", egui::Color32::from_rgb(220, 50, 50)),
                        Player::O => ("O", egui::Color32::from_rgb(50, 110, 220)),
                        Player::None => ("", egui::Color32::BLACK),
                    };

                    let can_click = self.engine.status == GameStatus::Ongoing && cell == Player::None;

                    let mut rich = egui::RichText::new(symbol).size((cell_size * 0.5).max(18.0));
                    if cell != Player::None {
                        rich = rich.color(color).strong();
                    }

                    let mut button = egui::Button::new(rich).min_size(egui::vec2(cell_size, cell_size));

                    // Highlight winning cells even if not clickable
                    if let Some([a, b, c]) = winning {
                        if idx == a || idx == b || idx == c {
                            button = button.fill(egui::Color32::from_rgb(240, 220, 120));
                        } else if !can_click {
                            button = button.fill(egui::Color32::DARK_GRAY);
                        }
                    } else if !can_click {
                        button = button.fill(egui::Color32::DARK_GRAY);
                    }

                    if ui.add(button).clicked() {
                        if can_click {
                            if let Some(pos) = Position::new(idx as u8) {
                                let _ = self.engine.handle_event(GameEvent::PlayMove(pos));
                            }
                        }
                    }
                }
            });
        }
    }
}

fn winning_line(board: &[Player; 9]) -> Option<[usize; 3]> {
    let winning_combinations: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];

    for combo in winning_combinations {
        if board[combo[0]] != Player::None
            && board[combo[0]] == board[combo[1]]
            && board[combo[1]] == board[combo[2]]
        {
            return Some(combo);
        }
    }
    None
}

fn compute_cell_size(avail: egui::Vec2) -> f32 {
    let cell_w = (avail.x - 24.0) / 3.0;
    let cell_h = ((avail.y - RESERVED_VERTICAL) / 3.0).max(MIN_CELL);
    cell_w.min(cell_h).clamp(MIN_CELL, MAX_CELL)
}

/// Run the app natively (desktop).
pub fn run_desktop() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport = egui::ViewportBuilder::default().with_inner_size([380.0, 500.0]);
    let _ = eframe::run_native(
        "Rust Tic-Tac-Toe",
        native_options,
        Box::new(|_cc| Box::new(TicTacToeApp::default())),
    );
}

#[cfg(test)]
mod gui_unit_tests {
        use super::*;
        use crate::engine::Player;

        #[test]
        fn compute_cell_size_min_clamps() {
                let avail = egui::vec2(100.0, 300.0);
                let s = compute_cell_size(avail);
                assert!(s >= MIN_CELL);
        }

        #[test]
        fn compute_cell_size_max_clamps() {
                let avail = egui::vec2(1200.0, 1000.0);
                let s = compute_cell_size(avail);
                assert!(s <= MAX_CELL);
        }

        #[test]
        fn compute_cell_size_expected_value() {
                let avail = egui::vec2(360.0, 500.0);
                let s = compute_cell_size(avail);
                // expected roughly (360 - 24) / 3 = 112
                assert!((s - 112.0).abs() < 2.0);
        }

        #[test]
        fn winning_line_detects_horizontal_and_diagonal() {
                let mut b = [Player::None; 9];
                b[0] = Player::X; b[1] = Player::X; b[2] = Player::X;
                assert_eq!(winning_line(&b), Some([0,1,2]));

                let mut b2 = [Player::None; 9];
                b2[0] = Player::O; b2[4] = Player::O; b2[8] = Player::O;
                assert_eq!(winning_line(&b2), Some([0,4,8]));
        }

        #[test]
        fn winning_line_none_when_no_winner() {
                let b = [Player::None; 9];
                assert_eq!(winning_line(&b), None);
        }
}

/*
For wasm (future):

- Add to `Cargo.toml`:
    eframe = { version = "...", features = ["wgpu", "glow", "wasm"] }

- Example start for web (not active now):
    #[cfg(target_arch = "wasm32")]
    pub fn start_web() {
            // eframe provides helpers (e.g. start_web) behind wasm feature flags.
            // See eframe docs for the exact web startup snippet.
    }

*/
