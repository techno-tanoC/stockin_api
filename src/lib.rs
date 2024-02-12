pub mod domain;
pub mod handler;
pub mod repo;

use std::sync::Arc;

use anyhow::Result;
use axum::{routing::get, Router};
use handler::item;
use sqlx::SqlitePool;

pub type AppState = Arc<State>;

pub struct State {
    pub pool: SqlitePool,
}

pub async fn new_app(database_url: &str) -> Result<Router> {
    let state = new_state(database_url).await?;
    let router = new_router(state);
    Ok(router)
}

#[rustfmt::skip]
pub fn new_router(state: AppState) -> Router {
    let item_router = Router::new()
        .route("/", get(item::index).post(item::create))
        .route("/:item_id", get(item::find).put(item::update).delete(item::delete));

    Router::new()
        .nest("/items", item_router)
        .with_state(state)
}

pub async fn new_state(database_url: &str) -> Result<AppState> {
    let pool = SqlitePool::connect(database_url).await?;
    let state = State { pool };
    Ok(Arc::new(state))
}
