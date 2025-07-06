use crate::api::error::{ApiError, ApiResult};
use crate::persistence::PersistentObject;
use giga_chess_api_types::body::login::LoginBody;
use giga_chess_api_types::body::register::RegisterBody;
use giga_chess_api_types::response::login::LoginResponse;
use giga_chess_api_types::response::message::MessageResponse;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::runtime::Runtime;

pub mod error;

#[derive(Debug)]
pub struct MultiplayerClient {
    client: Client,
    runtime: Runtime,
    server_url: Option<String>,
}

impl Default for MultiplayerClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            runtime: Runtime::new().unwrap(),
            server_url: None,
        }
    }
}

impl MultiplayerClient {
    pub fn get_server_url(&self) -> Option<&str> {
        self.server_url.as_deref()
    }

    pub fn set_server_url(&mut self, server_url: impl Into<String>) {
        self.server_url = Some(server_url.into());
    }

    pub fn is_ready(&self) -> bool {
        self.server_url.is_some()
    }

    fn spawn_request<T, F>(&self, request: RequestBuilder, callback: F)
    where
        T: DeserializeOwned + Send + 'static,
        F: FnOnce(ApiResult<T>) + Send + 'static,
    {
        self.runtime.spawn(async move {
            let result = match request.send().await {
                Ok(response) if response.status().is_success() => {
                    match response.json::<T>().await {
                        Ok(data) => Ok(data),
                        Err(error) => Err(error.into()),
                    }
                }
                Ok(response) => match response.status() {
                    StatusCode::BAD_REQUEST => Err(ApiError::BadRequest(
                        response.text().await.unwrap_or_default(),
                    )),
                    StatusCode::CONFLICT => Err(ApiError::Collision(
                        response.text().await.unwrap_or_default(),
                    )),
                    StatusCode::NOT_FOUND => Err(ApiError::NotFound(
                        response.text().await.unwrap_or_default(),
                    )),
                    StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized(
                        response.text().await.unwrap_or_default(),
                    )),
                    StatusCode::TOO_MANY_REQUESTS => Err(ApiError::RateLimited(
                        response.text().await.unwrap_or_default(),
                    )),
                    _ => Err(ApiError::Unexpected(format!(
                        "[{}]: {}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    ))),
                },
                Err(error) => {
                    let error_source = error
                        .source()
                        .map(|e| e.to_string())
                        .unwrap_or_else(|| error.to_string());

                    if error.is_builder() {
                        Err(ApiError::InvalidServerUrl)
                    } else if error.is_request() || error.is_connect() {
                        Err(ApiError::Connection(format!(
                            "{error} (source: {error_source})"
                        )))
                    } else if error.is_body() || error.is_decode() {
                        Err(ApiError::Communication(format!(
                            "{error} (source: {error_source})"
                        )))
                    } else if error.is_timeout() {
                        Err(ApiError::ConnectionTimeout)
                    } else {
                        Err(error.into())
                    }
                }
            };

            callback(result);
        });
    }

    pub fn register<F>(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
        invite_code: impl Into<String>,
        callback: F,
    ) where
        F: FnOnce(ApiResult<LoginResponse>) + Send + 'static,
    {
        let Some(server_url) = &self.server_url else {
            callback(Err(ApiError::MissingServerUrl));
            return;
        };

        let body = RegisterBody {
            username: username.into(),
            password: password.into(),
            invite_code: invite_code.into(),
        };

        let request = self
            .client
            .post(format!("{server_url}/register"))
            .json(&body);
        self.spawn_request(request, callback);
    }

    pub fn login<F>(&self, username: impl Into<String>, password: impl Into<String>, callback: F)
    where
        F: FnOnce(ApiResult<LoginResponse>) + Send + 'static,
    {
        let Some(server_url) = &self.server_url else {
            callback(Err(ApiError::MissingServerUrl));
            return;
        };

        let body = LoginBody {
            username: username.into(),
            password: password.into(),
        };

        let request = self.client.post(format!("{server_url}/login")).json(&body);
        self.spawn_request(request, callback);
    }

    pub fn ping<F>(&self, token: &str, callback: F)
    where
        F: FnOnce(ApiResult<MessageResponse>) + Send + 'static,
    {
        let Some(server_url) = &self.server_url else {
            callback(Err(ApiError::MissingServerUrl));
            return;
        };

        let request = self
            .client
            .post(format!("{server_url}/ping"))
            .bearer_auth(token);
        self.spawn_request(request, callback);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiplayerClientPersist {
    pub server_url: Option<String>,
}

impl PersistentObject for MultiplayerClient {
    type PersistentType = MultiplayerClientPersist;

    fn save_state(&self) -> Self::PersistentType {
        MultiplayerClientPersist {
            server_url: self.server_url.clone(),
        }
    }

    fn load_from_state(state: Self::PersistentType) -> Self {
        Self {
            server_url: state.server_url,
            ..Default::default()
        }
    }
}
