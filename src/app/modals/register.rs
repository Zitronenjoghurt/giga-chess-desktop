use crate::app::components::validated_field::ValidatedField;
use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::login::{LoginState, LoginStateStatus};
use crate::app::state::AppState;
use crate::app::validation::{validate_invite_code, validate_password, validate_username};
use egui::{Button, Id, Ui};

#[derive(Debug, Default)]
pub struct RegisterModal {
    username: String,
    password: String,
    invite_code: String,
    open: bool,
    just_opened: bool,
}

impl Modal for RegisterModal {
    fn id(&self) -> Id {
        Id::new("register_modal")
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
            .label_width(65.0)
            .validator(validate_username)
            .error_message("Username must be between 3 and 50 characters and only contain alphanumeric characters.")
            .show(ui);

        let password_response = ValidatedField::new("Password", &mut self.password)
            .label_width(65.0)
            .validator(validate_password)
            .error_message("Password must be at least 8 characters long.")
            .password()
            .show(ui);

        let invite_code_response = ValidatedField::new("Invite code", &mut self.invite_code)
            .label_width(65.0)
            .validator(validate_invite_code)
            .error_message("Invite code must be a valid uuid.")
            .show(ui);

        let login_status = state.login_state.lock().get_status();
        if login_status.is_not_successful_nor_loading()
            && ui
                .add_enabled(
                    username_response.is_valid
                        && password_response.is_valid
                        && invite_code_response.is_valid,
                    Button::new("Register"),
                )
                .clicked()
        {
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
