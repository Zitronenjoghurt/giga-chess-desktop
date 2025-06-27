use crate::api::error::{ApiError, ApiResult};
use giga_chess_api_types::body::login::LoginBody;
use giga_chess_api_types::response::login::LoginResponse;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::de::DeserializeOwned;
use tokio::runtime::Runtime;
use tokio::time::sleep;

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
            sleep(std::time::Duration::from_millis(3000)).await;
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
                    StatusCode::NOT_FOUND => Err(ApiError::NotFound(
                        response.text().await.unwrap_or_default(),
                    )),
                    StatusCode::UNAUTHORIZED => Err(ApiError::Unauthorized(
                        response.text().await.unwrap_or_default(),
                    )),
                    _ => Err(ApiError::Unexpected(format!(
                        "[{}]: {}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    ))),
                },
                Err(error) => Err(error.into()),
            };

            callback(result);
        });
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
}
