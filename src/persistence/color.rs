use crate::persistence::PersistentObject;
use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Color32Persist(u32);

impl PersistentObject for Color32 {
    type PersistentType = Color32Persist;

    fn save_state(&self) -> Self::PersistentType {
        Color32Persist(
            ((self.r() as u32) << 24)
                | ((self.g() as u32) << 16)
                | ((self.b() as u32) << 8)
                | (self.a() as u32),
        )
    }

    fn load_from_state(state: Self::PersistentType) -> Self {
        Self::from_rgba_unmultiplied(
            (state.0 >> 24) as u8,
            (state.0 >> 16) as u8,
            (state.0 >> 8) as u8,
            (state.0 >> 0) as u8,
        )
    }
}
