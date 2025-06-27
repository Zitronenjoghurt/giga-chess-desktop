use crate::app::modals::Modal;
use crate::app::state::login::LoginStateStatus;
use crate::app::state::AppState;
use egui::{Id, TextEdit, Ui};

#[derive(Debug, Default)]
pub struct LoginModal {
    open: bool,
    username: String,
    password: String,
}

impl Modal for LoginModal {
    fn id(&self) -> Id {
        Id::new("login_modal")
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
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

        let login_status = state.login_state.lock().get_status();

        if (login_status == LoginStateStatus::Idle
            || login_status == LoginStateStatus::InvalidCredentials
            || login_status == LoginStateStatus::Error)
            && ui.button("Login").clicked()
        {
            state.login(self.username.clone(), self.password.clone());
        }

        match login_status {
            LoginStateStatus::Idle => {}
            LoginStateStatus::Loading => {
                ui.spinner();
            }
            LoginStateStatus::Success => {
                ui.label("Successfully logged in");
            }
            LoginStateStatus::InvalidCredentials => {
                ui.label("Invalid credentials");
            }
            LoginStateStatus::Error => {
                if let Some(error) = state.login_state.lock().get_error() {
                    ui.label(error);
                }
            }
        }
    }
}
