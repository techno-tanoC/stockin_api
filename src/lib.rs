pub mod domain;
pub mod handler;
pub mod repo;

use std::sync::Arc;

use anyhow::Result;
use axum::{
    routing::{get, post},
    Router,
};
use handler::{item, not_found, query};
use reqwest::Client;
use sqlx::SqlitePool;
use tower_http::validate_request::ValidateRequestHeaderLayer;

pub type AppState = Arc<State>;

pub struct State {
    pub pool: SqlitePool,
    pub client: Client,
}

pub struct App;

impl App {
    pub async fn new_app(database_url: &str, token: &str) -> Result<Router> {
        let state = Self::new_state(database_url).await?;
        let auth_layer = ValidateRequestHeaderLayer::bearer(token);
        let router = Self::new_router(state).route_layer(auth_layer);
        Ok(router)
    }

    #[rustfmt::skip]
    pub fn new_router(state: AppState) -> Router {
        let item_router = Router::new()
            .route("/", get(item::index).post(item::create))
            .route("/:item_id", get(item::find).put(item::update).delete(item::delete));

        let query_router = Router::new()
            .route("/info", post(query::info));

        Router::new()
            .nest("/items", item_router)
            .nest("/query", query_router)
            .fallback(not_found)
            .with_state(state)
    }

    pub async fn new_state(database_url: &str) -> Result<AppState> {
        let pool = SqlitePool::connect(database_url).await?;
        let client = Client::new();
        let state = State { pool, client };
        Ok(Arc::new(state))
    }
}
