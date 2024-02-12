use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Clone)]
pub struct JsonData<T> {
    status: StatusCode,
    data: T,
}

impl<T> JsonData<T> {
    pub fn ok(data: T) -> Result<Self> {
        Ok(Self {
            status: StatusCode::OK,
            data,
        })
    }

    pub fn created(data: T) -> Result<Self> {
        Ok(Self {
            status: StatusCode::CREATED,
            data,
        })
    }
}

impl<T> IntoResponse for JsonData<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        let json = json!({
            "data": self.data,
        });
        (self.status, Json(json)).into_response()
    }
}

#[derive(Debug, Clone)]
pub struct NoContent;

impl NoContent {
    pub fn ok() -> Result<Self> {
        Ok(NoContent)
    }
}

impl IntoResponse for NoContent {
    fn into_response(self) -> Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, Clone)]
pub enum AppError {
    ServerError,
    ClientError { status: StatusCode, message: String },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::ServerError => {
                let json = json!({
                    "error": {
                        "status": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        "message": "INTERNAL SERVER ERROR",
                    }
                });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json)).into_response()
            }
            Self::ClientError { status, message } => {
                let json = json!({
                    "error": {
                        "status": status.as_u16(),
                        "message": message,
                    }
                });
                (status, Json(json)).into_response()
            }
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(_value: E) -> Self {
        Self::ServerError
    }
}

#[derive(Debug, Clone)]
pub struct NotFound;

impl From<NotFound> for AppError {
    fn from(_value: NotFound) -> Self {
        AppError::ClientError {
            status: StatusCode::NOT_FOUND,
            message: "NOT FOUND".to_string(),
        }
    }
}
