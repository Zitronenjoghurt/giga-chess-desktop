use crate::app::state::AppState;
use crate::traits::chess_board::ChessBoardTrait;
use egui::{Color32, Pos2, Rect, Sense, Ui, Vec2};
use giga_chess::prelude::{Color, Square};

#[derive(Debug)]
pub struct ChessBoardComponent {
    light_color: Color32,
    dark_color: Color32,
    selected_square: Option<Square>,
}

impl ChessBoardComponent {
    pub fn new() -> Self {
        Self {
            light_color: Color32::from_rgb(255, 247, 228),
            dark_color: Color32::from_rgb(135, 168, 137),
            selected_square: None,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        state: &mut AppState,
        perspective: Color,
        board: &mut dyn ChessBoardTrait,
    ) {
        let available_rect = ui.available_rect_before_wrap();
        let available_size = available_rect.width().min(available_rect.height());

        let square_size = available_size / 8.0;

        let (response, painter) =
            ui.allocate_painter(Vec2::new(available_size, available_size), Sense::hover());
        let board_rect = response.rect;

        let mut squares = Square::iter_top_bottom().collect::<Vec<_>>();
        if perspective == Color::Black {
            squares.reverse();
        };

        for square in squares {
            let file = square.get_file();
            let rank = square.get_rank();

            let x = board_rect.min.x + (file - 1) as f32 * square_size;
            let y = board_rect.min.y + (8 - rank) as f32 * square_size;

            let square_rect =
                Rect::from_min_size(Pos2::new(x, y), Vec2::new(square_size, square_size));
            let square_response = ui.allocate_rect(square_rect, Sense::click_and_drag());

            if square_response.clicked() {
                self.selected_square = Some(square);
                println!("Clicked on {} ({})", square, square.get_value());
            }

            let color = if square.is_white() {
                self.light_color
            } else {
                self.dark_color
            };
            painter.rect_filled(square_rect, 0.0, color);

            if let Some((piece, color)) = board.piece_color_at(square) {
                let image = state
                    .assets
                    .get_piece_image(ui.ctx(), piece, color, square_size);
                ui.put(square_rect, image);
            }
        }
    }
}
