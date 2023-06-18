use axum::{response::IntoResponse, Json};
use http::StatusCode;
use migration::DbErr;
use serde_json::json;

#[derive(Debug)]
pub struct ServiceDBError(pub DbErr);

impl IntoResponse for ServiceDBError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.0 {
            DbErr::Conn(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
        };

        let message = match self.0 {
            DbErr::Conn(message) => message.to_string(),
            _ => "Something went wrong".to_string(),
        };

        let body = Json(json!({
            "message": message,
        }));

        (status, body).into_response()
    }
}

impl From<migration::DbErr> for ServiceDBError {
    fn from(value: migration::DbErr) -> Self {
        value.into()
    }
}
