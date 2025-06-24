use crate::app::asset_server::AssetServer;
use crate::app::views::ViewID;
use giga_chess::prelude::Engine;
use std::sync::Arc;

#[derive(Debug, Default)]
pub struct AppState {
    pub assets: AssetServer,
    pub engine: Arc<Engine>,
    pub current_view: ViewID,
}
