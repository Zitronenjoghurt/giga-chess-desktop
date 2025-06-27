use crate::app::state::AppState;
use egui::{Context, Id, Ui};

pub mod login;
pub mod server_settings;

pub trait Modal {
    fn id(&self) -> Id;
    fn set_open(&mut self, open: bool);
    fn is_open(&self) -> bool;
    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState);
    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        if !self.is_open() {
            return;
        }

        let modal_response = egui::Modal::new(self.id()).show(ctx, |ui| {
            self.render_content(ui, state);
        });

        if modal_response.should_close() {
            self.set_open(false);
        }
    }
}
