use crate::app::components::chess_board::ChessBoardComponent;
use crate::app::state::AppState;
use crate::app::views::{View, ViewID};
use crate::app::windows::game_control::GameControlWindow;
use crate::app::windows::sandbox_game_creation::SandboxGameCreationWindow;
use crate::app::windows::{ToggleableWindow, ToggleableWindowWithData, WindowEvent};
use crate::game::AppGame;
use egui::{Button, Context, RichText, TopBottomPanel, Window};
use giga_chess::prelude::PGNMetadata;

#[derive(Debug, Default)]
pub struct SandboxView {
    chess_board: ChessBoardComponent,
    game: Option<AppGame>,
    game_creation_window: SandboxGameCreationWindow,
    game_control_window: GameControlWindow,
}

impl SandboxView {
    fn on_home_clicked(&mut self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::MainMenu);
    }
}

impl View for SandboxView {
    fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        let window_event = self.game_creation_window.render(ctx, state);
        if window_event == WindowEvent::CreateSandboxGame {
            self.game = Some(AppGame::new(&state.engine, PGNMetadata::now()));
            self.chess_board.clear();
            self.game_control_window.set_open(true);
        }

        TopBottomPanel::top("sandbox_top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let home_response = ui.add(Button::new(RichText::new(" 🏠 ").size(20.0)));
                if home_response.clicked() {
                    self.on_home_clicked(ctx, state);
                }

                let mut game_creation_open = self.game_creation_window.is_open();
                ui.checkbox(&mut game_creation_open, "Game Creation");
                self.game_creation_window.set_open(game_creation_open);

                let mut game_control_open = self.game_control_window.is_open();
                ui.checkbox(&mut game_control_open, "Game Control");
                self.game_control_window.set_open(game_control_open);
            });
        });

        if let Some(game) = &mut self.game {
            let _ = self.game_control_window.render_with_data(ctx, state, game);
            Window::new("Chess Board").show(ctx, |ui| {
                self.chess_board.render(ui, state, game);
            });
        }
    }
}
