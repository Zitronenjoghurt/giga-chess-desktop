use crate::api::error::ApiError;
use crate::api::{MultiplayerClient, MultiplayerClientPersist};
use crate::app::asset_server::AssetServer;
use crate::app::state::login::LoginState;
use crate::app::views::ViewID;
use crate::persistence::PersistentObject;
use crate::types::shared::Shared;
use giga_chess::prelude::Engine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod login;

#[derive(Debug, Default)]
pub struct AppState {
    pub api: MultiplayerClient,
    pub assets: AssetServer,
    pub engine: Arc<Engine>,
    pub current_view: ViewID,
    pub login_state: Shared<LoginState>,
}

impl AppState {
    pub fn switch_view(&mut self, view: ViewID) {
        self.current_view = view;
    }

    pub fn set_multiplayer_url(&mut self, url: impl Into<String>) {
        self.api.set_server_url(url);
    }

    pub fn login(&mut self, username: impl Into<String>, password: impl Into<String>) {
        let login_state = self.login_state.clone();
        login_state.set(LoginState::Loading);
        self.api
            .login(username, password, move |result| match result {
                Ok(response) => {
                    login_state.set(LoginState::Success(response.token));
                }
                Err(err) => match err {
                    ApiError::BadRequest(_) | ApiError::Unauthorized(_) => {
                        login_state.set(LoginState::error("Invalid username or password."))
                    }
                    _ => login_state.set(LoginState::Error(err.to_string())),
                },
            });
    }

    pub fn register(
        &mut self,
        username: impl Into<String>,
        password: impl Into<String>,
        invite_code: impl Into<String>,
    ) {
        let login_state = self.login_state.clone();
        login_state.set(LoginState::Loading);
        self.api.register(
            username,
            password,
            invite_code,
            move |result| match result {
                Ok(response) => {
                    login_state.set(LoginState::Success(response.token));
                }
                Err(err) => match err {
                    ApiError::BadRequest(error) => login_state.set(LoginState::error(error)),
                    ApiError::Collision(_) => login_state.set(LoginState::error(
                        "The username is already taken, please choose another one.",
                    )),
                    _ => login_state.set(LoginState::Error(err.to_string())),
                },
            },
        );
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppStatePersist {
    pub api: MultiplayerClientPersist,
    pub current_view: ViewID,
    pub login_state: LoginState,
}

impl PersistentObject for AppState {
    type PersistentType = AppStatePersist;

    fn save_state(&self) -> Self::PersistentType {
        AppStatePersist {
            api: self.api.save_state(),
            current_view: self.current_view,
            login_state: self.login_state.get_clone(),
        }
    }

    fn load_from_state(state: Self::PersistentType) -> Self {
        Self {
            api: MultiplayerClient::load_from_state(state.api),
            current_view: state.current_view,
            login_state: Shared::new(state.login_state),
            ..Default::default()
        }
    }
}
