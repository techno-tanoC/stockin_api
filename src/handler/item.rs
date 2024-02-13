use axum::extract::State;
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    domain::item::{Item, ItemParams},
    repo::item,
    AppState,
};

use super::{
    request::{Json, Path, Query},
    response::{JsonData, NotFound, Result},
};

#[derive(Debug, Clone, Deserialize)]
pub struct Id {
    item_id: Uuid,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Range {
    from: Option<Uuid>,
    limit: Option<i64>,
}

pub async fn find(state: State<AppState>, id: Path<Id>) -> Result<JsonData<Item>> {
    let item = item::find_by_id(&state.pool, id.item_id)
        .await?
        .ok_or(NotFound)?;
    JsonData::ok(item)
}

pub async fn index(state: State<AppState>, params: Query<Range>) -> Result<JsonData<Vec<Item>>> {
    let from = params.from.unwrap_or(Uuid::max());
    let limit = params.limit.unwrap_or(20).clamp(1, 50);
    let items = item::find_by_range(&state.pool, from, limit).await?;
    JsonData::ok(items)
}

pub async fn create(
    state: State<AppState>,
    Json(params): Json<ItemParams>,
) -> Result<JsonData<Item>> {
    let mut tx = state.pool.begin().await?;
    let id = item::insert(&mut *tx, params).await?;
    let item = item::find_by_id(&mut *tx, id).await?.ok_or(NotFound)?;
    tx.commit().await.unwrap();
    JsonData::created(item)
}

pub async fn update(
    state: State<AppState>,
    id: Path<Id>,
    Json(params): Json<ItemParams>,
) -> Result<JsonData<Item>> {
    let mut tx = state.pool.begin().await?;
    item::update(&mut *tx, id.item_id, params).await?;
    let item = item::find_by_id(&mut *tx, id.item_id)
        .await?
        .ok_or(NotFound)?;
    tx.commit().await?;
    JsonData::ok(item)
}

pub async fn delete(state: State<AppState>, id: Path<Id>) -> Result<JsonData<serde_json::Value>> {
    item::delete(&state.pool, id.item_id).await.unwrap();
    JsonData::empty()
}
