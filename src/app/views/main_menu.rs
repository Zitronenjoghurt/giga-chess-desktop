use crate::app::modals::login::LoginModal;
use crate::app::modals::server_settings::ServerSettingsModal;
use crate::app::modals::Modal;
use crate::app::state::AppState;
use crate::app::views::{View, ViewID};
use egui::{Button, CentralPanel, Context, RichText, Vec2};

#[derive(Debug, Default)]
pub struct MainMenuView {
    login_modal: LoginModal,
    server_settings_modal: ServerSettingsModal,
}

impl MainMenuView {
    fn on_sandbox_clicked(&mut self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::Sandbox)
    }

    fn on_online_clicked(&mut self, _ctx: &Context, _state: &mut AppState) {}
}

impl View for MainMenuView {
    fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.login_modal.render(ctx, state);
        self.server_settings_modal.render(ctx, state);

        CentralPanel::default().show(ctx, |ui| {
            let vertical_space = ui.available_size_before_wrap().y;
            ui.vertical_centered(|ui| {
                ui.add_space(vertical_space / 5.0);
                ui.heading(RichText::new("Giga Chess").size(100.0));

                ui.add_space(50.0);
                let sandbox_response = ui.add(
                    Button::new(RichText::new("Sandbox").size(50.0))
                        .min_size(Vec2::new(400.0, 100.0)),
                );

                ui.add_space(20.0);
                let online_response = ui.add(
                    Button::new(RichText::new("Online").size(50.0))
                        .min_size(Vec2::new(400.0, 100.0)),
                );

                if sandbox_response.clicked() {
                    self.on_sandbox_clicked(ctx, state);
                }
                if online_response.clicked() {
                    self.on_online_clicked(ctx, state);
                }
            });
        });
    }
}
