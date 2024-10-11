use axum::{http::StatusCode, response::IntoResponse};
pub struct AppError(StatusCode, String);

impl AppError {
    pub fn new(status_code: StatusCode, message: String) -> Self {
        Self(status_code, message)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, self.1).into_response()
    }
}

pub struct DtoError {
    pub message: String,
}

impl DtoError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Debug for DtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error: {}", self.message)
    }
}
