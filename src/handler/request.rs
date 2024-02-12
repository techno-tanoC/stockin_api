use std::ops::Deref;

use axum::{
    extract::{
        rejection::{JsonRejection, PathRejection, QueryRejection},
        FromRequest, FromRequestParts,
    },
    http::StatusCode,
    response::IntoResponse,
};

#[derive(FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ErrorRejection))]
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ErrorRejection))]
pub struct Path<T>(pub T);

impl<T> Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ErrorRejection))]
pub struct Query<T>(pub T);

impl<T> Deref for Query<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct ErrorRejection {
    status: StatusCode,
    message: String,
}

impl IntoResponse for ErrorRejection {
    fn into_response(self) -> axum::response::Response {
        let payload = serde_json::json!({
            "error": {
                "status": self.status.as_u16(),
                "message": self.message,
            }
        });
        (self.status, axum::Json(payload)).into_response()
    }
}

impl From<JsonRejection> for ErrorRejection {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<PathRejection> for ErrorRejection {
    fn from(rejection: PathRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

impl From<QueryRejection> for ErrorRejection {
    fn from(rejection: QueryRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}
