use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum LoginStateStatus {
    #[default]
    Idle,
    Loading,
    Success,
    Error,
}

impl LoginStateStatus {
    pub fn is_not_successful_nor_loading(&self) -> bool {
        *self != Self::Loading && *self != Self::Success
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub enum LoginState {
    #[default]
    Idle,
    Loading,
    Success(String),
    Error(String),
}

impl LoginState {
    pub fn get_status(&self) -> LoginStateStatus {
        match self {
            Self::Idle => LoginStateStatus::Idle,
            Self::Loading => LoginStateStatus::Loading,
            Self::Success(_) => LoginStateStatus::Success,
            Self::Error(_) => LoginStateStatus::Error,
        }
    }

    pub fn get_token(&self) -> Option<&str> {
        match self {
            Self::Success(token) => Some(token),
            _ => None,
        }
    }

    pub fn get_error(&self) -> Option<&str> {
        match self {
            Self::Error(error) => Some(error),
            _ => None,
        }
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self::Error(error.into())
    }
}
