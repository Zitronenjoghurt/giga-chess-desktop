use giga_chess_api_types::validation::alphanumeric::is_alphanumeric;
use giga_chess_api_types::validation::uuid::is_uuid;

pub fn validate_username(username: &str) -> bool {
    is_alphanumeric(username).is_ok() && username.len() >= 3 && username.len() <= 16
}

pub fn validate_password(password: &str) -> bool {
    password.len() >= 8 && password.len() <= 100
}

pub fn validate_invite_code(invite_code: &str) -> bool {
    is_uuid(invite_code).is_ok()
}
