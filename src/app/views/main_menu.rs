use crate::app::components::chess_board::ChessBoardComponent;
use crate::app::state::AppState;
use crate::app::views::View;
use egui::{Context, Window};
use giga_chess::prelude::{Game, PGNMetadata};

#[derive(Debug)]
pub struct MainMenuView {
    chess_board: ChessBoardComponent,
    game: Game,
}

impl View for MainMenuView {
    fn new(state: &AppState) -> Self {
        let game = Game::new(&state.engine, PGNMetadata::now());
        Self {
            chess_board: ChessBoardComponent::new(state, &game),
            game,
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        Window::new("Chess Board").show(ctx, |ui| {
            self.chess_board
                .render(ui, state, self.game.side_to_move(), &mut self.game);
        });
    }
}
