use crate::app::state::AppState;
use egui::{Context, Id, Ui};

pub mod login;
pub mod login_or_register;
pub mod register;
pub mod server_settings;

pub trait Modal {
    fn id(&self) -> Id;
    fn set_open(&mut self, open: bool);
    fn is_open(&self) -> bool;

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) -> ModalEvent {
        ModalEvent::None
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) -> ModalEvent {
        if !self.is_open() {
            return ModalEvent::None;
        }

        let mut modal_event = ModalEvent::None;
        let modal_response = egui::Modal::new(self.id()).show(ctx, |ui| {
            modal_event = self.render_content(ui, state);
        });

        if modal_response.should_close() {
            self.set_open(false);
        }

        modal_event
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ModalEvent {
    #[default]
    None,
    ChooseLogin,
    ChooseRegister,
    LoginSuccess,
    RegisterSuccess,
    SetServerSettings,
}
