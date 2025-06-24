use giga_chess::game::state::GameState;
use giga_chess::game::Game;
use giga_chess::prelude::{ChessBoard, Color, Piece, Square};

pub trait ChessBoardTrait: Send + Sync {
    fn piece_color_at(&self, square: Square) -> Option<(Piece, Color)>;
}

impl ChessBoardTrait for ChessBoard {
    fn piece_color_at(&self, square: Square) -> Option<(Piece, Color)> {
        self.get_piece_at(square.get_value())
    }
}

impl ChessBoardTrait for GameState {
    fn piece_color_at(&self, square: Square) -> Option<(Piece, Color)> {
        self.board.piece_color_at(square)
    }
}

impl ChessBoardTrait for Game {
    fn piece_color_at(&self, square: Square) -> Option<(Piece, Color)> {
        self.board().piece_color_at(square)
    }
}
