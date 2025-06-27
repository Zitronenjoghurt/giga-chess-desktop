use crate::app::state::AppState;
use crate::app::views::{View, ViewManager};
use eframe::{App, Frame};
use egui::Context;

mod asset_server;
mod components;
mod modals;
mod state;
mod views;

#[derive(Debug)]
pub struct GigaChessApp {
    state: AppState,
    view_manager: ViewManager,
}

impl GigaChessApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let state = AppState::default();
        let view_manager = ViewManager::new(&state);
        Self {
            state,
            view_manager,
        }
    }
}

impl App for GigaChessApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.view_manager.render(ctx, &mut self.state);
    }
}
