use crate::app::state::AppState;
use egui::{Color32, Id, Pos2, Rect, Sense, Ui, Vec2};
use giga_chess::prelude::{Color, Game, Square};

#[derive(Debug)]
pub struct ChessBoardComponent {
    light_color: Color32,
    light_color_highlight: Color32,
    dark_color: Color32,
    dark_color_highlight: Color32,
    dragging_from: Option<Square>,
    last_from: Option<Square>,
    last_to: Option<Square>,
}

impl ChessBoardComponent {
    pub fn new() -> Self {
        Self {
            light_color: Color32::from_rgb(255, 247, 228),
            light_color_highlight: Color32::from_rgb(255, 197, 178),
            dark_color: Color32::from_rgb(135, 168, 137),
            dark_color_highlight: Color32::from_rgb(200, 168, 137),
            dragging_from: None,
            last_from: None,
            last_to: None,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut Ui,
        state: &mut AppState,
        perspective: Color,
        game: &mut Game,
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
            let square_response = ui.allocate_rect(square_rect, Sense::drag());

            let color = self.get_square_color(square);
            painter.rect_filled(square_rect, 0.0, color);

            if let Some((piece, color)) = game.board().get_piece_at(square.get_value()) {
                let piece_id = Id::new(format!("piece_{}", square));
                ui.allocate_ui_at_rect(square_rect, |ui| {
                    ui.dnd_drag_source(piece_id, square, |ui| {
                        let image =
                            state
                                .assets
                                .get_piece_image(ui.ctx(), piece, color, square_size);
                        ui.add(image);
                    });
                });

                if ui.ctx().is_being_dragged(piece_id) {
                    self.dragging_from = Some(square);
                } else {
                    self.dragging_from = None;
                }
            }

            if let Some(dragged_square) = square_response.dnd_release_payload::<Square>() {
                self.on_drag_drop(*dragged_square, square, game, state);
            }
        }
    }

    fn get_square_color(&self, square: Square) -> Color32 {
        let is_highlight = Some(square) == self.last_from || Some(square) == self.last_to;

        if square.is_white() {
            if is_highlight {
                self.light_color_highlight
            } else {
                self.light_color
            }
        } else if is_highlight {
            self.dark_color_highlight
        } else {
            self.dark_color
        }
    }

    fn on_drag_drop(&mut self, from: Square, to: Square, game: &mut Game, state: &AppState) {
        let success = game.play_move_from_to(&state.engine, from, to, None);
        if success {
            self.last_from = Some(from);
            self.last_to = Some(to);
        }
    }
}
