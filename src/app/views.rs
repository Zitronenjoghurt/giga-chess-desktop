mod main_menu;

use crate::app::state::AppState;
use crate::app::views::main_menu::MainMenuView;
use egui::Context;

#[derive(Debug, Default, Copy, Clone)]
pub enum ViewID {
    #[default]
    MainMenu,
}

pub trait View {
    fn new(state: &AppState) -> Self;
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Debug)]
pub struct ViewManager {
    main_menu: MainMenuView,
}

impl View for ViewManager {
    fn new(state: &AppState) -> Self {
        Self {
            main_menu: MainMenuView::new(state),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view {
            ViewID::MainMenu => self.main_menu.render(ctx, state),
        }
    }
}
