use crate::app::components::validated_field::ValidatedField;
use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::login::{LoginState, LoginStateStatus};
use crate::app::state::AppState;
use crate::app::validation::{validate_password, validate_username};
use egui::{Button, Id, Ui};

#[derive(Debug, Default)]
pub struct LoginModal {
    open: bool,
    just_opened: bool,
    username: String,
    password: String,
}

impl Modal for LoginModal {
    fn id(&self) -> Id {
        Id::new("login_modal")
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
        self.just_opened = open;
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) -> ModalEvent {
        if self.just_opened {
            state.login_state.set(LoginState::Idle);
            self.just_opened = false;
        }

        let username_response = ValidatedField::new("Username", &mut self.username)
            .label_width(60.0)
            .validator(validate_username)
            .error_message("Username must be between 3 and 50 characters and only contain alphanumeric characters.")
            .show(ui);

        let password_response = ValidatedField::new("Password", &mut self.password)
            .label_width(60.0)
            .validator(validate_password)
            .error_message("Password must be at least 8 characters long.")
            .password()
            .show(ui);

        let login_status = state.login_state.lock().get_status();
        if login_status.is_not_successful_nor_loading()
            && ui
                .add_enabled(
                    username_response.is_valid && password_response.is_valid,
                    Button::new("Login"),
                )
                .clicked()
        {
            state.login(&self.username, &self.password);
        }

        let mut event = ModalEvent::None;
        match login_status {
            LoginStateStatus::Idle => {}
            LoginStateStatus::Loading => {
                ui.spinner();
            }
            LoginStateStatus::Success => {
                ui.label("Successfully logged in");
                event = ModalEvent::LoginSuccess;
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
