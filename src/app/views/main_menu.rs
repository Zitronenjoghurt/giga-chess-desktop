use crate::app::modals::login::LoginModal;
use crate::app::modals::login_or_register::LoginOrRegisterModal;
use crate::app::modals::register::RegisterModal;
use crate::app::modals::server_settings::ServerSettingsModal;
use crate::app::modals::{Modal, ModalEvent};
use crate::app::state::login::LoginStateStatus;
use crate::app::state::AppState;
use crate::app::views::{View, ViewID};
use egui::{Button, CentralPanel, Context, RichText, Vec2};

#[derive(Debug, Default)]
pub struct MainMenuView {
    login_modal: LoginModal,
    login_or_register_modal: LoginOrRegisterModal,
    register_modal: RegisterModal,
    server_settings_modal: ServerSettingsModal,
}

impl MainMenuView {
    fn render_login_modal(&mut self, ctx: &Context, state: &mut AppState) {
        let event = self.login_modal.render(ctx, state);
        if event == ModalEvent::LoginSuccess {
            self.login_modal.set_open(false);
            state.switch_view(ViewID::OnlineHome);
        }
    }

    fn render_login_or_register_modal(&mut self, ctx: &Context, state: &mut AppState) {
        let event = self.login_or_register_modal.render(ctx, state);
        if event == ModalEvent::ChooseLogin {
            self.login_or_register_modal.set_open(false);
            self.login_modal.set_open(true);
        } else if event == ModalEvent::ChooseRegister {
            self.login_or_register_modal.set_open(false);
            self.register_modal.set_open(true);
        }
    }

    fn render_register_modal(&mut self, ctx: &Context, state: &mut AppState) {
        let event = self.register_modal.render(ctx, state);
        if event == ModalEvent::RegisterSuccess {
            self.register_modal.set_open(false);
            state.switch_view(ViewID::OnlineHome);
        }
    }

    fn render_server_settings_modal(&mut self, ctx: &Context, state: &mut AppState) {
        let event = self.server_settings_modal.render(ctx, state);
        if event == ModalEvent::SetServerSettings {
            self.server_settings_modal.set_open(false);
            self.login_or_register_modal.set_open(true);
        }
    }

    fn on_sandbox_clicked(&mut self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::Sandbox)
    }

    fn on_online_clicked(&mut self, _ctx: &Context, state: &mut AppState) {
        if (!state.api.is_ready()) {
            self.server_settings_modal.set_open(true);
        } else if (state.login_state.lock().get_status() != LoginStateStatus::Success) {
            self.login_or_register_modal.set_open(true);
        } else {
            state.switch_view(ViewID::OnlineHome);
        }
    }
}

impl View for MainMenuView {
    fn new() -> Self {
        Self::default()
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        self.render_login_modal(ctx, state);
        self.render_login_or_register_modal(ctx, state);
        self.render_register_modal(ctx, state);
        self.render_server_settings_modal(ctx, state);

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
