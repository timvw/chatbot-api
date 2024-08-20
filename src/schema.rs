use crate::model::AppError;
use axum::response::IntoResponse;
use http::StatusCode;

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}
