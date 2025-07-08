use crate::app::state::{AppState, AppStatePersist};
use crate::app::views::{View, ViewManager};
use crate::persistence;
use crate::persistence::PersistentObject;
use eframe::{App, Frame};
use egui::Context;
use serde::{Deserialize, Serialize};

mod asset_server;
mod components;
mod modals;
mod state;
mod validation;
mod views;
mod windows;

#[derive(Debug)]
pub struct GigaChessApp {
    state: AppState,
    view_manager: ViewManager,
}

impl Default for GigaChessApp {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            view_manager: ViewManager::new(),
        }
    }
}

impl GigaChessApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(state) = persistence::restore() {
            Self::load_from_state(state)
        } else {
            Self::default()
        }
    }
}

impl App for GigaChessApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.view_manager.render(ctx, &mut self.state);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let persistent_state = self.save_state();
        persistence::persist(persistent_state);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GigaChessAppPersist {
    state: AppStatePersist,
}

impl PersistentObject for GigaChessApp {
    type PersistentType = GigaChessAppPersist;

    fn save_state(&self) -> GigaChessAppPersist {
        GigaChessAppPersist {
            state: self.state.save_state(),
        }
    }

    fn load_from_state(state: GigaChessAppPersist) -> Self {
        Self {
            state: AppState::load_from_state(state.state),
            ..Default::default()
        }
    }
}
