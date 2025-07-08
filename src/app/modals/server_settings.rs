use crate::app::components::validated_field::ValidatedField;
use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::AppState;
use crate::app::validation::validate_url;
use egui::{Button, Id, Ui};

#[derive(Debug, Default)]
pub struct ServerSettingsModal {
    open: bool,
    url: String,
    initialized: bool,
}

impl Modal for ServerSettingsModal {
    fn id(&self) -> Id {
        Id::new("server_settings_modal")
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) -> ModalEvent {
        if !self.initialized {
            self.url = state.api.get_server_url().unwrap_or_default().to_string();
            self.initialized = true;
        }

        let server_url_response = ValidatedField::new("Server URL", &mut self.url)
            .label_width(60.0)
            .validator(validate_url)
            .error_message("Must be a valid URL.")
            .show(ui);

        let mut event = ModalEvent::None;
        if ui
            .add_enabled(server_url_response.is_valid, Button::new("Save"))
            .clicked()
        {
            state.api.set_server_url(&self.url);
            self.set_open(false);
            event = ModalEvent::SetServerSettings;
        }

        event
    }
}
