mod main_menu;
mod sandbox;

use crate::app::state::AppState;
use crate::app::views::main_menu::MainMenuView;
use egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    MainMenu,
}

pub trait View {
    fn new() -> Self;
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Debug)]
pub struct ViewManager {
    main_menu: MainMenuView,
}

impl View for ViewManager {
    fn new() -> Self {
        Self {
            main_menu: MainMenuView::new(),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view {
            ViewID::MainMenu => self.main_menu.render(ctx, state),
        }
    }
}
