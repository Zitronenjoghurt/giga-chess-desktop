use crate::app::modals::Modal;
use crate::app::state::AppState;
use egui::{Id, Ui};

#[derive(Debug, Default)]
pub struct ServerSettingsModal {
    open: bool,
    url: String,
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

    fn render_content(&mut self, ui: &mut Ui, state: &mut AppState) {
        ui.horizontal(|ui| {
            ui.label("Server URL");
            ui.text_edit_singleline(&mut self.url);
        });

        if ui.button("Save").clicked() {
            state.api.set_server_url(&self.url);
            self.set_open(false);
        }
    }
}
