use crate::app::state::AppState;
use crate::app::windows::{ToggleableWindow, ToggleableWindowWithData, WindowEvent};
use crate::game::AppGame;
use egui::{ComboBox, Grid, Id, Ui, WidgetText};
use giga_chess::prelude::{Color, Piece};

const PROMOTION_PIECES: [Piece; 4] = [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight];

#[derive(Debug, Default)]
pub struct GameControlWindow {
    open: bool,
}

impl ToggleableWindow for GameControlWindow {
    fn id(&self) -> Id {
        Id::new("game_control_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Game Control"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }
}

impl ToggleableWindowWithData<AppGame> for GameControlWindow {
    fn render_content_with_data(
        &mut self,
        ui: &mut Ui,
        _state: &mut AppState,
        app_game: &mut AppGame,
    ) -> WindowEvent {
        Grid::new("game_info")
            .num_columns(2)
            .spacing([40.0, 4.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Status:");
                ui.label(format!("{:?}", app_game.game.status()));
                ui.end_row();

                ui.label("Turn:");
                ui.label(format!("{:?}", app_game.game.side_to_move()));
                ui.end_row();

                ui.label("Winner:");
                ui.label(
                    app_game
                        .game
                        .winner()
                        .map(|winner| format!("{winner:?}"))
                        .unwrap_or_else(|| "None".to_string()),
                );
                ui.end_row();
            });

        ui.horizontal(|ui| {
            ui.label("Perspective:");
            ui.radio_value(&mut app_game.perspective, Color::White, "White");
            ui.radio_value(&mut app_game.perspective, Color::Black, "Black");
        });

        ui.horizontal(|ui| {
            ui.label("Promotion piece:");
            ComboBox::from_id_salt("promotion_piece_combo")
                .selected_text(format!("{:?}", app_game.promotion_piece))
                .show_ui(ui, |ui| {
                    for piece in PROMOTION_PIECES {
                        ui.selectable_value(
                            &mut app_game.promotion_piece,
                            piece,
                            format!("{piece:?}"),
                        );
                    }
                });
        });

        ui.horizontal(|ui| {
            ui.label("Auto perspective:");
            let mut auto_adjust_perspective = app_game.get_auto_adjust_perspective();
            ui.checkbox(&mut auto_adjust_perspective, "");
            app_game.set_auto_adjust_perspective(auto_adjust_perspective);
        });

        WindowEvent::None
    }
}
