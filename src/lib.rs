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
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::validate_request::ValidateRequestHeaderLayer;

pub type AppState = Arc<State>;

pub struct State {
    pub pool: PgPool,
    pub client: Client,
}

impl State {
    pub async fn from_url(database_url: &str) -> Result<AppState> {
        let pool = PgPool::connect(database_url).await?;
        let client = Client::new();
        let state = Self { pool, client };
        Ok(Arc::new(state))
    }

    pub async fn from_pool(pool: PgPool) -> AppState {
        let client = Client::new();
        let state = Self { pool, client };
        Arc::new(state)
    }
}

pub struct App {
    router: Router,
}

impl App {
    pub async fn new(state: AppState, token: &str) -> Result<Self> {
        let auth_layer = ValidateRequestHeaderLayer::bearer(token);
        let router = Self::build_router(state).route_layer(auth_layer);
        Ok(Self { router })
    }

    pub async fn start(self, listener: TcpListener) -> Result<()> {
        axum::serve(listener, self.router).await?;
        Ok(())
    }

    #[rustfmt::skip]
    pub fn build_router(state: AppState) -> Router {
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
}
