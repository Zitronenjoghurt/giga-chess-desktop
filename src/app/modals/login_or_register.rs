use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::AppState;
use egui::{Id, Ui};

#[derive(Debug, Default)]
pub struct LoginOrRegisterModal {
    open: bool,
}

impl Modal for LoginOrRegisterModal {
    fn id(&self) -> Id {
        Id::new("login_or_register_modal")
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn render_content(&mut self, ui: &mut Ui, _state: &mut AppState) -> ModalEvent {
        ui.vertical_centered(|ui| ui.label("Would you like to login or register?"));

        let mut event = ModalEvent::None;
        ui.vertical_centered_justified(|ui| {
            if ui.button("Login").clicked() {
                event = ModalEvent::ChooseLogin;
            }

            if ui.button("Register").clicked() {
                event = ModalEvent::ChooseRegister;
            }
        });

        event
    }
}
