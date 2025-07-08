mod main_menu;
mod online_home;
mod sandbox;

use crate::app::state::AppState;
use crate::app::views::main_menu::MainMenuView;
use crate::app::views::online_home::OnlineHomeView;
use crate::app::views::sandbox::SandboxView;
use egui::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub enum ViewID {
    #[default]
    MainMenu,
    OnlineHome,
    Sandbox,
}

pub trait View {
    fn new() -> Self;
    fn render(&mut self, ctx: &Context, state: &mut AppState);
}

#[derive(Debug)]
pub struct ViewManager {
    main_menu: MainMenuView,
    online_home: OnlineHomeView,
    sandbox: SandboxView,
}

impl View for ViewManager {
    fn new() -> Self {
        Self {
            main_menu: MainMenuView::new(),
            online_home: OnlineHomeView::new(),
            sandbox: SandboxView::new(),
        }
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) {
        match state.current_view {
            ViewID::MainMenu => self.main_menu.render(ctx, state),
            ViewID::OnlineHome => self.online_home.render(ctx, state),
            ViewID::Sandbox => self.sandbox.render(ctx, state),
        }
    }
}
