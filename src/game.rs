use giga_chess::prelude::{Color, Engine, Game, PGNMetadata, Piece};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppGame {
    pub game: Game,
    pub promotion_piece: Piece,
    pub perspective: Color,
    pub played_color: Option<Color>,
}

impl AppGame {
    pub fn new(engine: &Arc<Engine>, pgn: PGNMetadata) -> Self {
        Self {
            game: Game::new(engine, pgn),
            promotion_piece: Piece::Queen,
            perspective: Color::White,
            played_color: None,
        }
    }

    pub fn can_color_move(&self, color: Color) -> bool {
        Some(color) == self.played_color || self.played_color.is_none()
    }
}
