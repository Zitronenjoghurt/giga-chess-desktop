use eframe::emath::Vec2;
use egui::{Context, Image};
use giga_chess::prelude::{Color, Piece};
use std::collections::HashMap;

pub const SVG_B_B: &[u8] = include_bytes!("../../assets/b_b.svg");
pub const SVG_B_W: &[u8] = include_bytes!("../../assets/b_w.svg");
pub const SVG_K_B: &[u8] = include_bytes!("../../assets/k_b.svg");
pub const SVG_K_W: &[u8] = include_bytes!("../../assets/k_w.svg");
pub const SVG_N_B: &[u8] = include_bytes!("../../assets/n_b.svg");
pub const SVG_N_W: &[u8] = include_bytes!("../../assets/n_w.svg");
pub const SVG_P_B: &[u8] = include_bytes!("../../assets/p_b.svg");
pub const SVG_P_W: &[u8] = include_bytes!("../../assets/p_w.svg");
pub const SVG_R_B: &[u8] = include_bytes!("../../assets/r_b.svg");
pub const SVG_R_W: &[u8] = include_bytes!("../../assets/r_w.svg");
pub const SVG_Q_B: &[u8] = include_bytes!("../../assets/q_b.svg");
pub const SVG_Q_W: &[u8] = include_bytes!("../../assets/q_w.svg");

#[derive(Debug, Default)]
pub struct AssetServer {
    last_piece_size: HashMap<u8, u32>,
}

impl AssetServer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_piece_image(
        &mut self,
        ctx: &Context,
        piece: Piece,
        color: Color,
        size: f32,
    ) -> Image {
        let svg_size = size as u32;
        let piece_color_key = piece as u8 + color as u8 * 6;

        let do_reload = if let Some(last_size) = self.last_piece_size.get(&piece_color_key) {
            *last_size != svg_size
        } else {
            true
        };

        let uri = self.get_piece_uri(piece, color);
        let image = if do_reload {
            ctx.forget_image(&uri);
            self.last_piece_size.insert(piece_color_key, svg_size);
            Image::from_bytes(uri, self.get_piece_bytes(piece, color))
        } else {
            Image::from_uri(uri)
        };

        image.fit_to_exact_size(Vec2::splat(svg_size as f32))
    }

    fn get_piece_uri(&self, piece: Piece, color: Color) -> String {
        format!(
            "{}_{}.svg",
            piece.get_char().to_ascii_lowercase(),
            color.get_fen_char()
        )
    }

    fn get_piece_bytes(&self, piece: Piece, color: Color) -> &'static [u8] {
        match (piece, color) {
            (Piece::Pawn, Color::Black) => SVG_P_B,
            (Piece::Pawn, Color::White) => SVG_P_W,
            (Piece::Knight, Color::Black) => SVG_N_B,
            (Piece::Knight, Color::White) => SVG_N_W,
            (Piece::Bishop, Color::Black) => SVG_B_B,
            (Piece::Bishop, Color::White) => SVG_B_W,
            (Piece::Rook, Color::Black) => SVG_R_B,
            (Piece::Rook, Color::White) => SVG_R_W,
            (Piece::Queen, Color::Black) => SVG_Q_B,
            (Piece::Queen, Color::White) => SVG_Q_W,
            (Piece::King, Color::Black) => SVG_K_B,
            (Piece::King, Color::White) => SVG_K_W,
        }
    }
}
