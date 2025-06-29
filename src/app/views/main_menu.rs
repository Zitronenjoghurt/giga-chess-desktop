use crate::app::modals::login::LoginModal;
use crate::app::modals::server_settings::ServerSettingsModal;
use crate::app::modals::Modal;
use crate::app::state::AppState;
use crate::app::views::View;
use egui::{CentralPanel, Context};

#[derive(Debug, Default)]
pub struct MainMenuView {
    login_modal: LoginModal,
    server_settings_modal: ServerSettingsModal,
}

impl View for MainMenuView {
    fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.login_modal.render(ctx, state);
        self.server_settings_modal.render(ctx, state);

        CentralPanel::default().show(ctx, |ui| {
            if ui.button("Server").clicked() {
                self.server_settings_modal.set_open(true);
            }
            if ui.button("Login").clicked() {
                self.login_modal.set_open(true);
            }
        });
    }
}
