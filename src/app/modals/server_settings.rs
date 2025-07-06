use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::AppState;
use egui::{Button, Id, Ui};
use url::Url;

#[derive(Debug, Default)]
pub struct ServerSettingsModal {
    open: bool,
    url: String,
    initialized: bool,
}

impl ServerSettingsModal {
    pub fn validate_url(&self) -> bool {
        Url::parse(&self.url).is_ok()
    }
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

        let url_valid = self.validate_url();

        ui.horizontal(|ui| {
            ui.label("Server URL");
            ui.text_edit_singleline(&mut self.url);
            if url_valid {
                ui.label("✅");
            } else {
                ui.colored_label(ui.visuals().error_fg_color, "❌");
            }
        });

        let mut event = ModalEvent::None;
        if ui.add_enabled(url_valid, Button::new("Save")).clicked() {
            state.api.set_server_url(&self.url);
            self.set_open(false);
            event = ModalEvent::SetServerSettings;
        }

        event
    }
}
