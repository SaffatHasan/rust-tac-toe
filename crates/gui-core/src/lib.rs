//! eframe/egui Tic-Tac-Toe: Refined UI with stable layout and dark aesthetic.

use eframe::{egui, App};
use rust_tac_toe_engine::{GameEngine, GameStatus, Player, Position};

// Style Constants
const BOARD_SIZE: f32 = 360.0;
const CELL_GAP: f32 = 12.0;
const CORNER_RADIUS: f32 = 10.0;

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
        self.apply_terminal_theme(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            // 1. Fixed Header Area (Prevents board from moving)
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("TIC-TAC-TOE")
                        .size(32.0)
                        .strong()
                        .extra_letter_spacing(2.0),
                );

                // Fixed height container for status so it doesn't push the board down
                ui.allocate_ui(egui::vec2(ui.available_width(), 40.0), |ui| {
                    ui.centered_and_justified(|ui| {
                        self.render_status(ui);
                    });
                });
            });

            ui.add_space(20.0);

            // 2. Centered Board with Fixed Dimensions
            self.render_centered_board(ui);

            // 3. Footer
            ui.add_space(30.0);
            ui.vertical_centered(|ui| {
                if ui
                    .add(
                        egui::Button::new(egui::RichText::new("NEW GAME").size(16.0).strong())
                            .fill(egui::Color32::from_rgb(50, 50, 50))
                            .min_size(egui::vec2(120.0, 40.0)),
                    )
                    .clicked()
                {
                    self.engine.reset();
                }
            });
        });
    }
}

impl TicTacToeApp {
    fn apply_terminal_theme(&self, ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();

        // Deep charcoal/navy background, not pure black
        visuals.panel_fill = egui::Color32::from_rgb(18, 18, 22);
        visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(30, 30, 35);
        visuals.widgets.inactive.bg_fill = egui::Color32::from_rgb(40, 40, 45);
        visuals.widgets.hovered.bg_fill = egui::Color32::from_rgb(50, 50, 60);

        ctx.set_visuals(visuals);

        let mut style: egui::Style = (*ctx.style()).clone();
        style.visuals.widgets.inactive.rounding = CORNER_RADIUS.into();
        ctx.set_style(style);
    }

    fn render_status(&self, ui: &mut egui::Ui) {
        let (text, color) = match self.engine.status {
            GameStatus::Ongoing => match self.engine.current_player {
                Player::X => ("X'S TURN", egui::Color32::from_rgb(255, 85, 85)),
                Player::O => ("O'S TURN", egui::Color32::from_rgb(85, 170, 255)),
            },
            GameStatus::Win { player, .. } => (
                "WINNER!",
                if player == Player::X {
                    egui::Color32::from_rgb(255, 85, 85)
                } else {
                    egui::Color32::from_rgb(85, 170, 255)
                },
            ),
            GameStatus::Draw => ("DRAW", egui::Color32::LIGHT_GRAY),
        };
        ui.label(egui::RichText::new(text).size(20.0).strong().color(color));
    }

    fn render_centered_board(&mut self, ui: &mut egui::Ui) {
        let cell_size = (BOARD_SIZE - (2.0 * CELL_GAP)) / 3.0;

        // Calculate the total width of the grid to center it manually
        let total_grid_width = (cell_size * 3.0) + (CELL_GAP * 2.0);
        let horizontal_padding = (ui.available_width() - total_grid_width) / 2.0;

        ui.horizontal(|ui| {
            // Push the grid to the center by adding space on the left
            ui.add_space(horizontal_padding);

            egui::Grid::new("ttt_grid")
                .spacing(egui::vec2(CELL_GAP, CELL_GAP))
                .show(ui, |ui| {
                    let winning_line = if let GameStatus::Win { line, .. } = self.engine.status {
                        Some(line)
                    } else {
                        None
                    };

                    for row in 0..3 {
                        for col in 0..3 {
                            let idx = row * 3 + col;
                            let is_win =
                                winning_line.map_or(false, |l| l.contains(&(idx as usize)));
                            self.render_cell(ui, idx, is_win, cell_size);
                        }
                        ui.end_row();
                    }
                });
        });
    }

    fn render_cell(&mut self, ui: &mut egui::Ui, idx: usize, is_win: bool, size: f32) {
        let cell = self.engine.board[idx];
        let (symbol, color) = match cell {
            Some(Player::X) => ("X", egui::Color32::from_rgb(255, 85, 85)),
            Some(Player::O) => ("O", egui::Color32::from_rgb(85, 170, 255)),
            None => ("", egui::Color32::TRANSPARENT),
        };

        let can_click = self.engine.status == GameStatus::Ongoing && cell.is_none();

        let mut button = egui::Button::new(
            egui::RichText::new(symbol)
                .size(size * 0.6)
                .strong()
                .color(color),
        )
        .min_size(egui::vec2(size, size));

        // Highlight winning line with a subtle glow, otherwise keep dark
        if is_win {
            button = button.fill(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 20));
            button = button.stroke(egui::Stroke::new(2.0, color));
        } else {
            button = button.fill(egui::Color32::from_rgb(35, 35, 40));
        }

        if ui
            .add_enabled(can_click || cell.is_some(), button)
            .clicked()
            && can_click
        {
            if let Some(pos) = Position::new(idx as u8) {
                let _ = self.engine.play_move(pos);
            }
        }
    }
}
