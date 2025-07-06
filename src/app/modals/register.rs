use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::login::LoginStateStatus;
use crate::app::state::AppState;
use egui::{Id, TextEdit, Ui};

#[derive(Debug, Default)]
pub struct RegisterModal {
    username: String,
    password: String,
    invite_code: String,
    open: bool,
}

impl Modal for RegisterModal {
    fn id(&self) -> Id {
        Id::new("register_modal")
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) -> ModalEvent {
        ui.horizontal(|ui| {
            ui.label("Username");
            ui.text_edit_singleline(&mut self.username);
        });

        ui.horizontal(|ui| {
            ui.label("Password");
            TextEdit::singleline(&mut self.password)
                .password(true)
                .show(ui);
        });

        ui.horizontal(|ui| {
            ui.label("Invite Code");
            ui.text_edit_singleline(&mut self.invite_code);
        });

        let login_status = state.login_state.lock().get_status();
        if login_status.is_not_successful_nor_loading() && ui.button("Register").clicked() {
            state.register(&self.username, &self.password, &self.invite_code);
        }

        let mut event = ModalEvent::None;
        match login_status {
            LoginStateStatus::Idle => {}
            LoginStateStatus::Loading => {
                ui.spinner();
            }
            LoginStateStatus::Success => {
                ui.label("Successfully registered");
                event = ModalEvent::RegisterSuccess;
            }
            LoginStateStatus::Error => {
                if let Some(error) = state.login_state.lock().get_error() {
                    ui.label(error);
                }
            }
        }

        event
    }
}
