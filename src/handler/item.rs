use axum::extract::State;

use crate::{
    AppState,
    domain::item::{Item, ItemId, ItemParams, ItemRange},
    repo::item,
};

use super::{
    request::{Json, Path, Query},
    response::{JsonData, NotFound, Result},
};

pub async fn find(state: State<AppState>, id: Path<ItemId>) -> Result<JsonData<Item>> {
    let item = item::find_by_id(&state.pool, id.item_id)
        .await?
        .ok_or(NotFound)?;
    JsonData::ok(item)
}

pub async fn index(
    state: State<AppState>,
    params: Query<ItemRange>,
) -> Result<JsonData<Vec<Item>>> {
    let (before, limit) = params.extract();
    let items = item::find_by_range(&state.pool, before, limit).await?;
    JsonData::ok(items)
}

pub async fn create(
    state: State<AppState>,
    Json(params): Json<ItemParams>,
) -> Result<JsonData<Item>> {
    let mut tx = state.pool.begin().await?;
    let id = item::insert(&mut *tx, params).await?;
    let item = item::find_by_id(&mut *tx, id).await?.ok_or(NotFound)?;
    tx.commit().await?;
    JsonData::created(item)
}

pub async fn update(
    state: State<AppState>,
    id: Path<ItemId>,
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

pub async fn delete(
    state: State<AppState>,
    id: Path<ItemId>,
) -> Result<JsonData<serde_json::Value>> {
    item::delete(&state.pool, id.item_id).await?;
    JsonData::empty()
}
