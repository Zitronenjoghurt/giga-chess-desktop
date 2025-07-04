use crate::app::state::AppState;
use crate::app::windows::{ToggleableWindow, ToggleableWindowWithData, WindowEvent};
use crate::game::AppGame;
use egui::{ComboBox, Id, Ui, WidgetText};
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
        ui.label(format!("Color to move: {:?}", app_game.game.side_to_move()));

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

        WindowEvent::None
    }
}
