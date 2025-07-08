use crate::app::state::AppState;
use crate::app::views::{View, ViewID};
use egui::{Button, CentralPanel, Context, RichText, TopBottomPanel};

#[derive(Debug, Default)]
pub struct OnlineHomeView;

impl OnlineHomeView {
    fn on_home_clicked(&mut self, _ctx: &Context, state: &mut AppState) {
        state.switch_view(ViewID::MainMenu);
    }
}

impl View for OnlineHomeView {
    fn new() -> Self {
        Self
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        // Just for the background color
        CentralPanel::default().show(ctx, |_ui| {});

        TopBottomPanel::top("online_home_top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let home_response = ui.add(Button::new(RichText::new(" 🏠 ").size(20.0)));
                if home_response.clicked() {
                    self.on_home_clicked(ctx, state);
                }

                ui.label("Online Mode");
            });
        });
    }
}
