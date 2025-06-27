#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum LoginStateStatus {
    #[default]
    Idle,
    Loading,
    Success,
    InvalidCredentials,
    Error,
}

#[derive(Debug, Default, Clone)]
pub enum LoginState {
    #[default]
    Idle,
    Loading,
    Success(String),
    InvalidCredentials,
    Error(String),
}

impl LoginState {
    pub fn get_status(&self) -> LoginStateStatus {
        match self {
            Self::Idle => LoginStateStatus::Idle,
            Self::Loading => LoginStateStatus::Loading,
            Self::Success(_) => LoginStateStatus::Success,
            Self::InvalidCredentials => LoginStateStatus::InvalidCredentials,
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
}
