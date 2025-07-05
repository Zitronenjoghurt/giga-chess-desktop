use giga_chess::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppGame {
    pub game: Game,
    pub promotion_piece: Piece,
    pub perspective: Color,
    pub played_color: Option<Color>,
    auto_adjust_perspective: bool,
}

impl AppGame {
    pub fn new(engine: &Arc<Engine>, pgn: PGNMetadata) -> Self {
        Self {
            game: Game::new(engine, pgn),
            promotion_piece: Piece::Queen,
            perspective: Color::White,
            played_color: None,
            auto_adjust_perspective: false,
        }
    }

    pub fn try_play_move(&mut self, engine: &Arc<Engine>, from: Square, to: Square) -> bool {
        let (moving_piece, moving_color) =
            self.game.board().get_piece_at(from.get_value()).unwrap();

        let move_promotion_piece =
            if moving_piece == Piece::Pawn && to.is_promotion_square(moving_color) {
                Some(self.promotion_piece)
            } else {
                None
            };

        let success = self
            .game
            .play_move_from_to(engine, from, to, move_promotion_piece);

        if success && self.auto_adjust_perspective {
            self.adjust_perspective();
        }

        success
    }

    pub fn can_color_move(&self, color: Color) -> bool {
        if self.game.status() != GameStatus::Running {
            return false;
        }
        Some(color) == self.played_color || self.played_color.is_none()
    }

    pub fn get_auto_adjust_perspective(&self) -> bool {
        self.auto_adjust_perspective
    }

    pub fn set_auto_adjust_perspective(&mut self, auto_adjust_perspective: bool) {
        self.auto_adjust_perspective = auto_adjust_perspective;
        if auto_adjust_perspective {
            self.adjust_perspective();
        }
    }

    fn adjust_perspective(&mut self) {
        self.perspective = self.game.side_to_move();
    }
}
