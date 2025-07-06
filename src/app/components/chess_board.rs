use crate::app::state::AppState;
use crate::game::AppGame;
use crate::persistence::color::Color32Persist;
use crate::persistence::PersistentObject;
use egui::epaint::CircleShape;
use egui::{Align2, Color32, FontId, Id, Painter, Pos2, Rect, Sense, Stroke, Ui, Vec2};
use giga_chess::prelude::{Color, Square};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug)]
pub struct ChessBoardComponent {
    light_color: Color32,
    light_color_highlight: Color32,
    light_color_threat: Color32,
    dark_color: Color32,
    dark_color_highlight: Color32,
    dark_color_threat: Color32,
    dragging_from: Option<Square>,
    threat_squares: Vec<Square>,
    target_square_map: HashMap<Square, Vec<Square>>,
    dirty: bool,
}

impl Default for ChessBoardComponent {
    fn default() -> Self {
        Self::new()
    }
}

impl ChessBoardComponent {
    pub fn new() -> Self {
        Self {
            light_color: Color32::from_rgb(255, 247, 228),
            light_color_highlight: Color32::from_rgb(255, 197, 178),
            light_color_threat: Color32::from_rgb(249, 130, 132),
            dark_color: Color32::from_rgb(135, 168, 137),
            dark_color_highlight: Color32::from_rgb(200, 168, 137),
            dark_color_threat: Color32::from_rgb(199, 103, 105),
            dragging_from: None,
            threat_squares: Vec::new(),
            target_square_map: HashMap::new(),
            dirty: true,
        }
    }

    pub fn clear(&mut self) {
        self.dirty = true;
    }

    pub fn render(&mut self, ui: &mut Ui, state: &mut AppState, app_game: &mut AppGame) {
        if self.dirty {
            self.threat_squares = app_game.game.get_check_threats(&state.engine);
            self.target_square_map = app_game.game.legal_move_squares();
            self.dirty = false;
        }

        let available_rect = ui.available_rect_before_wrap();
        let available_size = available_rect.width().min(available_rect.height());
        let square_size = available_size / 9.0;

        let (response, painter) =
            ui.allocate_painter(Vec2::new(available_size, available_size), Sense::hover());
        let board_rect = response.rect;

        for square in Square::iter_top_bottom() {
            self.render_square(
                ui,
                state,
                app_game,
                &painter,
                board_rect,
                square,
                square_size,
            );
        }

        self.render_file_rank(&painter, app_game.perspective, board_rect, square_size);
    }

    #[allow(clippy::too_many_arguments)]
    fn render_square(
        &mut self,
        ui: &mut Ui,
        state: &mut AppState,
        app_game: &mut AppGame,
        painter: &Painter,
        board_rect: Rect,
        square: Square,
        square_size: f32,
    ) {
        let (x, y) =
            self.get_square_coordinates(square, app_game.perspective, board_rect, square_size);

        let square_rect = Rect::from_min_size(Pos2::new(x, y), Vec2::new(square_size, square_size));
        let square_response = ui.allocate_rect(square_rect, Sense::drag());

        let last_move = app_game.game.latest_move();
        let last_from = last_move.map(|mv| Square::new(mv.get_from()));
        let last_to = last_move.map(|mv| Square::new(mv.get_to()));
        let color = self.get_square_color(square, last_from, last_to);
        painter.rect_filled(square_rect, 0.0, color);

        self.render_piece(ui, state, app_game, square, square_rect, square_size);

        if let Some(dragged_square) = square_response.dnd_release_payload::<Square>() {
            self.on_drag_drop(*dragged_square, square, app_game, state);
        }

        if let Some(dragging_from) = self.dragging_from {
            if let Some(target_squares) = self.target_square_map.get(&dragging_from) {
                if target_squares.contains(&square) {
                    painter.add(self.get_target_circle(square_rect));
                }
            }
        }
    }

    fn render_piece(
        &mut self,
        ui: &mut Ui,
        state: &mut AppState,
        app_game: &mut AppGame,
        square: Square,
        square_rect: Rect,
        square_size: f32,
    ) {
        if let Some((piece, color)) = app_game.game.board().get_piece_at(square.get_value()) {
            let piece_id = Id::new(format!("piece_{square}"));
            ui.allocate_ui_at_rect(square_rect, |ui| {
                if app_game.can_color_move(color) {
                    let drag_inner = ui.dnd_drag_source(piece_id, square, |ui| {
                        let image =
                            state
                                .assets
                                .get_piece_image(ui.ctx(), piece, color, square_size);
                        ui.add(image);
                    });

                    if drag_inner.response.drag_stopped() {
                        self.dragging_from = None;
                    }
                } else {
                    let image = state
                        .assets
                        .get_piece_image(ui.ctx(), piece, color, square_size);
                    ui.add(image);
                }
            });

            if ui.ctx().is_being_dragged(piece_id) {
                self.dragging_from = Some(square);
            }
        }
    }

    fn render_file_rank(
        &mut self,
        painter: &Painter,
        perspective: Color,
        board_rect: Rect,
        square_size: f32,
    ) {
        let font_id = FontId::proportional(square_size * 0.35);
        let text_color = Color32::from_gray(100);

        for file in 1..=8 {
            let file_char = if perspective == Color::White {
                (b'A' + file - 1) as char
            } else {
                (b'A' + 8 - file) as char
            };

            let x = board_rect.min.x + (file as f32) * square_size;
            let bottom_pos = Pos2::new(x, board_rect.max.y);
            let top_pos = Pos2::new(x, board_rect.min.y);

            painter.text(
                bottom_pos,
                Align2::CENTER_BOTTOM,
                file_char,
                font_id.clone(),
                text_color,
            );

            painter.text(
                top_pos,
                Align2::CENTER_TOP,
                file_char,
                font_id.clone(),
                text_color,
            );
        }

        for rank in 1..=8 {
            let rank_char = if perspective == Color::White {
                (b'1' + rank - 1) as char
            } else {
                (b'1' + 8 - rank) as char
            };

            let y = board_rect.min.y + (9 - rank) as f32 * square_size;
            let left_pos = Pos2::new(board_rect.min.x + square_size * 0.125, y);
            let right_pos = Pos2::new(board_rect.max.x - square_size * 0.125, y);

            painter.text(
                left_pos,
                Align2::LEFT_CENTER,
                rank_char,
                font_id.clone(),
                text_color,
            );

            painter.text(
                right_pos,
                Align2::RIGHT_CENTER,
                rank_char,
                font_id.clone(),
                text_color,
            );
        }
    }

    fn get_square_coordinates(
        &self,
        square: Square,
        perspective: Color,
        board_rect: Rect,
        square_size: f32,
    ) -> (f32, f32) {
        let file = square.get_file();
        let rank = square.get_rank();

        let x = if perspective == Color::White {
            board_rect.min.x + (file as f32 - 0.5) * square_size
        } else {
            board_rect.min.x + (8.5 - file as f32) * square_size
        };

        let y = if perspective == Color::White {
            board_rect.min.y + (8.5 - rank as f32) * square_size
        } else {
            board_rect.min.y + (rank as f32 - 0.5) * square_size
        };

        (x, y)
    }

    fn get_square_color(
        &self,
        square: Square,
        last_from: Option<Square>,
        last_to: Option<Square>,
    ) -> Color32 {
        let is_highlight = Some(square) == last_from || Some(square) == last_to;
        let is_threat = self.threat_squares.contains(&square);

        if square.is_white() {
            if is_threat {
                self.light_color_threat
            } else if is_highlight {
                self.light_color_highlight
            } else {
                self.light_color
            }
        } else if is_threat {
            self.dark_color_threat
        } else if is_highlight {
            self.dark_color_highlight
        } else {
            self.dark_color
        }
    }

    fn get_target_circle(&self, square_rect: Rect) -> CircleShape {
        let center = square_rect.center();
        let radius = square_rect.width() / 7.0;
        CircleShape {
            center,
            radius,
            fill: self.light_color,
            stroke: Stroke::new(radius / 3.0, self.dark_color),
        }
    }

    fn on_drag_drop(&mut self, from: Square, to: Square, app_game: &mut AppGame, state: &AppState) {
        self.dragging_from = None;
        let success = app_game.try_play_move(&state.engine, from, to);
        if success {
            self.dirty = true;
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChessBoardComponentPersist {
    light_color: Color32Persist,
    light_color_highlight: Color32Persist,
    light_color_threat: Color32Persist,
    dark_color: Color32Persist,
    dark_color_highlight: Color32Persist,
    dark_color_threat: Color32Persist,
}

impl PersistentObject for ChessBoardComponent {
    type PersistentType = ChessBoardComponentPersist;

    fn save_state(&self) -> Self::PersistentType {
        ChessBoardComponentPersist {
            light_color: self.light_color.save_state(),
            light_color_highlight: self.light_color_highlight.save_state(),
            light_color_threat: self.light_color_threat.save_state(),
            dark_color: self.dark_color.save_state(),
            dark_color_highlight: self.dark_color_highlight.save_state(),
            dark_color_threat: self.dark_color_threat.save_state(),
        }
    }

    fn load_from_state(state: Self::PersistentType) -> Self {
        Self {
            light_color: Color32::load_from_state(state.light_color),
            light_color_highlight: Color32::load_from_state(state.light_color_highlight),
            light_color_threat: Color32::load_from_state(state.light_color_threat),
            dark_color: Color32::load_from_state(state.dark_color),
            dark_color_highlight: Color32::load_from_state(state.dark_color_highlight),
            dark_color_threat: Color32::load_from_state(state.dark_color_threat),
            dragging_from: None,
            threat_squares: vec![],
            target_square_map: Default::default(),
            dirty: true,
        }
    }
}
