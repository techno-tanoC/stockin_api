use axum::extract::State;
use tracing::{error, instrument};

use crate::{
    domain::item::{Item, ItemId, ItemParams, ItemRange},
    repo::item,
    AppState,
};

use super::{
    request::{Json, Path, Query},
    response::{JsonData, NotFound, Result},
};

#[instrument(skip(state))]
pub async fn find(state: State<AppState>, Path(id): Path<ItemId>) -> Result<JsonData<Item>> {
    let item = item::find_by_id(&state.pool, id.item_id)
        .await?
        .ok_or(NotFound)?;
    JsonData::ok(item)
}

#[instrument(skip(state))]
pub async fn index(
    state: State<AppState>,
    Query(params): Query<ItemRange>,
) -> Result<JsonData<Vec<Item>>> {
    let (before, limit) = params.extract();
    let items = item::find_by_range(&state.pool, before, limit).await?;
    JsonData::ok(items)
}

#[instrument(skip(state))]
pub async fn create(
    state: State<AppState>,
    Json(params): Json<ItemParams>,
) -> Result<JsonData<Item>> {
    let mut tx = state.pool.begin().await.inspect_err(|e| error!("{}", e))?;
    let id = item::insert(&mut *tx, params).await?;
    let item = item::find_by_id(&mut *tx, id).await?.ok_or(NotFound)?;
    tx.commit().await.inspect_err(|e| error!("{}", e))?;
    JsonData::created(item)
}

#[instrument(skip(state))]
pub async fn update(
    state: State<AppState>,
    Path(id): Path<ItemId>,
    Json(params): Json<ItemParams>,
) -> Result<JsonData<Item>> {
    let mut tx = state.pool.begin().await.inspect_err(|e| error!("{}", e))?;
    item::update(&mut *tx, id.item_id, params).await?;
    let item = item::find_by_id(&mut *tx, id.item_id)
        .await?
        .ok_or(NotFound)?;
    tx.commit().await.inspect_err(|e| error!("{}", e))?;
    JsonData::ok(item)
}

#[instrument(skip(state))]
pub async fn delete(
    state: State<AppState>,
    Path(id): Path<ItemId>,
) -> Result<JsonData<serde_json::Value>> {
    item::delete(&state.pool, id.item_id)
        .await
        .inspect_err(|e| error!("{}", e))?;
    JsonData::empty()
}
