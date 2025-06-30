use crate::app::state::AppState;
use crate::app::windows::{ToggleableWindow, WindowEvent};
use egui::{Id, Ui, WidgetText};

#[derive(Debug, Default)]
pub struct SandboxGameCreationWindow {
    open: bool,
}

impl ToggleableWindow for SandboxGameCreationWindow {
    fn id(&self) -> Id {
        Id::new("sandbox_game_creation_window")
    }

    fn title(&self) -> impl Into<WidgetText> {
        "Sandbox Game Creation"
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    fn render_content(&mut self, ui: &mut Ui, _state: &mut AppState) -> WindowEvent {
        let mut event = WindowEvent::None;

        if ui.button("Create Game").clicked() {
            event = WindowEvent::CreateSandboxGame
        };

        event
    }
}
