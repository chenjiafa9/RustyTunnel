use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
}

impl ApiError {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        ApiError {
            code,
            message: message.into(),
        }
    }

    pub fn invalid_credentials() -> Self {
        ApiError::new(401, "Invalid username or password")
    }

    pub fn server_error(msg: impl Into<String>) -> Self {
        ApiError::new(500, msg)
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        ApiError::new(404, msg)
    }

    pub fn bad_request(msg: impl Into<String>) -> Self {
        ApiError::new(400, msg)
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
