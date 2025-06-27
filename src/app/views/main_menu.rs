use crate::app::components::chess_board::ChessBoardComponent;
use crate::app::modals::login::LoginModal;
use crate::app::modals::server_settings::ServerSettingsModal;
use crate::app::modals::Modal;
use crate::app::state::AppState;
use crate::app::views::View;
use egui::{CentralPanel, Context, Window};
use giga_chess::prelude::{Game, PGNMetadata};

#[derive(Debug)]
pub struct MainMenuView {
    chess_board: ChessBoardComponent,
    game: Game,
    login_modal: LoginModal,
    server_settings_modal: ServerSettingsModal,
}

impl View for MainMenuView {
    fn new(state: &AppState) -> Self {
        let game = Game::new(&state.engine, PGNMetadata::now());
        Self {
            chess_board: ChessBoardComponent::new(state, &game),
            game,
            login_modal: LoginModal::default(),
            server_settings_modal: ServerSettingsModal::default(),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.login_modal.render(ctx, state);
        self.server_settings_modal.render(ctx, state);

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Server").clicked() {
                self.server_settings_modal.set_open(true);
            }
            if ui.button("Login").clicked() {
                self.login_modal.set_open(true);
            }
        });

        Window::new("Chess Board").show(ctx, |ui| {
            self.chess_board
                .render(ui, state, self.game.side_to_move(), &mut self.game);
        });
    }
}
