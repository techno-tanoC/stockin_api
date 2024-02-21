use anyhow::Result;
use sqlx::SqliteExecutor;
use uuid::{fmt::Hyphenated, Uuid};

use crate::domain::item::{Item, ItemParams};

use super::{item_model::*, model_ext::ModelExt as _};

pub async fn find_by_id(
    exe: impl SqliteExecutor<'_>,
    id: impl Into<Hyphenated>,
) -> Result<Option<Item>> {
    let id = id.into();

    let opt = sqlx::query_as!(Model, r#"SELECT * FROM items WHERE id = ?"#, id)
        .fetch_optional(exe)
        .await?;

    opt.convert()
}

pub async fn find_by_range(
    exe: impl SqliteExecutor<'_>,
    before: impl Into<Hyphenated>,
    limit: i64,
) -> Result<Vec<Item>> {
    let before = before.into();

    let models = sqlx::query_as!(
        Model,
        r#"SELECT * FROM items WHERE id < ? ORDER BY id DESC LIMIT ?"#,
        before,
        limit,
    )
    .fetch_all(exe)
    .await?;

    models.convert()
}

pub async fn insert(exe: impl SqliteExecutor<'_>, params: ItemParams) -> Result<Uuid> {
    let model = InsertModel::new(params)?;

    sqlx::query!(
        r#"INSERT INTO items VALUES (?, ?, ?, ?, ?, ?)"#,
        model.id,
        model.title,
        model.url,
        model.thumbnail,
        model.created_at,
        model.updated_at,
    )
    .execute(exe)
    .await?;

    Ok(model.id.into_uuid())
}

pub async fn update(
    exe: impl SqliteExecutor<'_>,
    id: impl Into<Hyphenated>,
    params: ItemParams,
) -> Result<()> {
    let id = id.into();
    let model = UpdateModel::new(id, params)?;

    sqlx::query!(
        r#"UPDATE items SET title = ?, url = ?, thumbnail = ?, updated_at = ? WHERE id = ?"#,
        model.title,
        model.url,
        model.thumbnail,
        model.updated_at,
        model.id,
    )
    .execute(exe)
    .await?;

    Ok(())
}

pub async fn delete(exe: impl SqliteExecutor<'_>, id: impl Into<Hyphenated>) -> Result<()> {
    let id = id.into();

    sqlx::query!(r#"DELETE FROM items WHERE id = ?"#, id)
        .execute(exe)
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn save(pool: impl SqliteExecutor<'_>, item: &Item) {
    let id = item.id.hyphenated();
    sqlx::query!(
        r#"INSERT INTO items Values (?, ?, ?, ?, ?, ?)"#,
        id,
        item.title,
        item.url,
        item.thumbnail,
        item.created_at,
        item.updated_at,
    )
    .execute(pool)
    .await
    .unwrap();
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, DurationRound as _, Utc};

    use crate::repo::test_util;

    use super::*;

    #[tokio::test]
    async fn test_crud() {
        let pool = test_util::connect().await;

        let title = "example".to_string();
        let url = "https://example.com/".to_string();
        let thumbnail = "https://example.com/thumbnail.png".to_string();

        // Insert
        let id = {
            let insert_params = ItemParams {
                title: title.clone(),
                url: url.clone(),
                thumbnail: thumbnail.clone(),
            };
            insert(&pool, insert_params).await.unwrap()
        };

        // Find
        let item = {
            let item = find_by_id(&pool, id).await.unwrap().unwrap();
            assert_eq!(item.id, id);
            assert_eq!(item.title, title);
            assert_eq!(item.url, url);
            assert_eq!(item.thumbnail, thumbnail);
            assert_eq!(item.created_at, item.updated_at);
            item
        };

        // Update
        {
            let update_params = ItemParams {
                title: "updated example".to_string(),
                url: "https://updated-example.com/".to_string(),
                thumbnail: "https://updated-example.com/thumbnail.png".to_string(),
            };
            update(&pool, id, update_params.clone()).await.unwrap();
            let updated = find_by_id(&pool, id).await.unwrap().unwrap();
            assert_eq!(updated.id, id);
            assert_eq!(updated.title, update_params.title);
            assert_eq!(updated.url, update_params.url);
            assert_eq!(updated.thumbnail, update_params.thumbnail);
            assert_eq!(updated.created_at, item.created_at);
            assert_ne!(updated.updated_at, item.updated_at);
        }

        // Delete
        {
            delete(&pool, id).await.unwrap();
            let opt = find_by_id(&pool, id).await.unwrap();
            assert_eq!(opt, None);
        }
    }

    #[tokio::test]
    async fn test_find_by_range() {
        let pool = test_util::connect().await;

        // No items
        {
            let items = find_by_range(&pool, Uuid::max(), 100).await.unwrap();
            assert_eq!(items.len(), 0);
        }

        let mut ids = (0..10).map(|_| Uuid::now_v7()).collect::<Vec<_>>();

        // UUID v7はミリ秒単位で同じタイミングで生成るとすると時系列順にならないため、明示的にソートする
        ids.sort_unstable();

        for (i, id) in ids.iter().enumerate() {
            let now = Utc::now()
                .duration_trunc(Duration::microseconds(1))
                .unwrap();
            let item = Item {
                id: *id,
                title: format!("example{}", i),
                url: format!("https://example{}.com/", i),
                thumbnail: format!("https://example{}.com/", i),
                created_at: now,
                updated_at: now,
            };
            save(&pool, &item).await;
        }
        ids.reverse();

        // data  :   |----------|
        // before: |--------------|
        {
            let items = find_by_range(&pool, Uuid::max(), 100).await.unwrap();
            assert_eq!(items.len(), 10);
            assert_eq!(items[0].title, "example9".to_string());
            assert_eq!(items[9].title, "example0".to_string());
        }

        // data  :   |----------|
        // before: |--------|
        {
            let items = find_by_range(&pool, Uuid::max(), 5).await.unwrap();
            assert_eq!(items.len(), 5);
            assert_eq!(items[0].title, "example9".to_string());
            assert_eq!(items[4].title, "example5".to_string());
        }

        // data  : |----------|
        // before:    |----------|
        {
            let items = find_by_range(&pool, ids[3], 100).await.unwrap();
            assert_eq!(items.len(), 6);
            assert_eq!(items[0].title, "example5".to_string());
            assert_eq!(items[5].title, "example0".to_string());
        }

        // data  : |----------|
        // before:   |------|
        {
            let items = find_by_range(&pool, ids[3], 3).await.unwrap();
            assert_eq!(items.len(), 3);
            assert_eq!(items[0].title, "example5".to_string());
            assert_eq!(items[2].title, "example3".to_string());
        }

        // data  : |----------|
        // before:               |------|
        {
            let items = find_by_range(&pool, Uuid::nil(), 3).await.unwrap();
            assert_eq!(items.len(), 0);
        }
    }
}
