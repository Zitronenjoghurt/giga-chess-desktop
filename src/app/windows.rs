use crate::app::state::AppState;
use egui::{Context, Id, Ui, WidgetText};

pub mod game_control;
pub mod sandbox_game_creation;

pub trait ToggleableWindow {
    fn id(&self) -> Id;
    fn title(&self) -> impl Into<WidgetText>;
    fn is_open(&self) -> bool;
    fn set_open(&mut self, open: bool);

    fn render_content(&mut self, _ui: &mut Ui, _state: &mut AppState) -> WindowEvent {
        WindowEvent::None
    }

    fn render(&mut self, ctx: &Context, state: &mut AppState) -> WindowEvent {
        if !self.is_open() {
            return WindowEvent::None;
        }

        let mut is_open = self.is_open();
        let mut event = WindowEvent::None;

        egui::Window::new(self.title())
            .id(self.id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                event = self.render_content(ui, state);
            });

        self.set_open(is_open);
        event
    }

    fn resizable(&self) -> bool {
        true
    }

    fn movable(&self) -> bool {
        true
    }

    fn collapsible(&self) -> bool {
        true
    }
}

pub trait ToggleableWindowWithData<Data>: ToggleableWindow {
    fn render_content_with_data(
        &mut self,
        _ui: &mut Ui,
        _state: &mut AppState,
        _data: &mut Data,
    ) -> WindowEvent {
        WindowEvent::None
    }

    fn render_with_data(
        &mut self,
        ctx: &Context,
        state: &mut AppState,
        data: &mut Data,
    ) -> WindowEvent {
        if !self.is_open() {
            return WindowEvent::None;
        }

        let mut is_open = self.is_open();
        let mut event = WindowEvent::None;

        egui::Window::new(self.title())
            .id(self.id())
            .open(&mut is_open)
            .resizable(self.resizable())
            .collapsible(self.collapsible())
            .movable(self.movable())
            .show(ctx, |ui| {
                event = self.render_content_with_data(ui, state, data);
            });

        self.set_open(is_open);
        event
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WindowEvent {
    #[default]
    None,
    CreateSandboxGame,
}
