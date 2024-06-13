pub mod item;
pub mod query;
mod request;
mod response;

use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::Response};

use self::response::{AppError, NotFound};

pub async fn not_found() -> AppError {
    NotFound.into()
}

#[tracing::instrument(
    skip_all,
    fields(
        http_request.request_method = request.method().to_string(),
        http_request.request_url = request.uri().path().to_string(),
    )
)]
pub async fn access_log_middleware(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let url = request.uri().path().to_string();

    let now = Instant::now();
    let response = next.run(request).await;
    let latency = format!("{}s", now.elapsed().as_secs_f32());

    let status = response.status().as_u16();
    tracing::info!(
        http_request.request_method = method,
        http_request.request_url = url,
        http_request.status = status,
        http_request.latency = latency,
        "access_log",
    );

    response
}
