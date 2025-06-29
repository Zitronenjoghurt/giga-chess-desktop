use crate::app::components::chess_board::ChessBoardComponent;
use crate::app::state::AppState;
use crate::app::views::View;
use egui::{Context, Window};
use giga_chess::game::Game;

#[derive(Debug)]
pub struct SandboxView {
    chess_board: ChessBoardComponent,
    game: Option<Game>,
}

impl View for SandboxView {
    fn new() -> Self {
        Self {
            chess_board: ChessBoardComponent::default(),
            game: None,
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        if let Some(game) = &mut self.game {
            Window::new("Chess Board").show(ctx, |ui| {
                self.chess_board
                    .render(ui, state, game.side_to_move(), game);
            });
        }
    }
}
