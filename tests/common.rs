use axum::{
    Router,
    body::Body,
    http::{
        Method, Response, header,
        request::{Builder, Request},
    },
};
use http_body_util::BodyExt;
use stockin_api::{App, State};
use tower::ServiceExt as _;

pub struct TestApp {
    router: Router,
}

impl TestApp {
    pub async fn new() -> Self {
        let state = State::from_url("sqlite://:memory:").await.unwrap();
        let schema = tokio::fs::read_to_string("./schema.sql").await.unwrap();
        sqlx::query(&schema).execute(&state.pool).await.unwrap();
        let router = App::new_router(state);
        Self { router }
    }

    fn base_request() -> Builder {
        Request::builder().header(header::CONTENT_TYPE, "application/json")
    }

    pub async fn get(&self, path: &str) -> Response<Body> {
        let req = Self::base_request()
            .method(Method::GET)
            .uri(path)
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub async fn post(&self, path: &str, body: Body) -> Response<Body> {
        let req = Self::base_request()
            .method(Method::POST)
            .uri(path)
            .body(body)
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub async fn put(&self, path: &str, body: Body) -> Response<Body> {
        let req = Self::base_request()
            .method(Method::PUT)
            .uri(path)
            .body(body)
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }

    pub async fn delete(&self, path: &str) -> Response<Body> {
        let req = Self::base_request()
            .method(Method::DELETE)
            .uri(path)
            .body(Body::empty())
            .unwrap();
        self.router.clone().oneshot(req).await.unwrap()
    }
}

pub async fn into_json(res: Response<Body>) -> serde_json::Value {
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}
