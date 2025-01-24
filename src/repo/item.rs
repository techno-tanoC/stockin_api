use anyhow::Result;
use sqlx::PgExecutor;
use uuid::Uuid;

use crate::domain::item::{Item, ItemParams};

use super::{item_model::*, model_ext::ModelExt as _};

pub async fn find_by_id(
    exe: impl PgExecutor<'_>,
    id: impl Into<Uuid>,
) -> Result<Option<Item>> {
    let id = id.into();

    let opt = sqlx::query_as!(
        Model,
        r#"SELECT * FROM items WHERE id = $1"#,
        id,
    )
    .fetch_optional(exe)
    .await?;

    opt.convert()
}

pub async fn find_by_range(
    exe: impl PgExecutor<'_>,
    before: impl Into<Uuid>,
    limit: i64,
) -> Result<Vec<Item>> {
    let before = before.into();

    let models = sqlx::query_as!(
        Model,
        r#"SELECT * FROM items WHERE id < $1 ORDER BY id DESC LIMIT $2"#,
        before,
        limit,
    )
    .fetch_all(exe)
    .await?;

    models.convert()
}

pub async fn insert(exe: impl PgExecutor<'_>, params: ItemParams) -> Result<Uuid> {
    let model = InsertModel::new(params)?;

    sqlx::query!(
        r#"INSERT INTO items VALUES ($1, $2, $3, $4, $5, $6)"#,
        model.id,
        model.title,
        model.url,
        model.thumbnail,
        model.created_at,
        model.updated_at,
    )
    .execute(exe)
    .await?;

    Ok(model.id)
}

pub async fn update(
    exe: impl PgExecutor<'_>,
    id: impl Into<Uuid>,
    params: ItemParams,
) -> Result<()> {
    let id = id.into();
    let model = UpdateModel::new(id, params)?;

    sqlx::query!(
        r#"UPDATE items SET title = $1, url = $2, thumbnail = $3, updated_at = $4 WHERE id = $5"#,
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

pub async fn delete(exe: impl PgExecutor<'_>, id: impl Into<Uuid>) -> Result<()> {
    let id = id.into();

    sqlx::query!(r#"DELETE FROM items WHERE id = $1"#, id)
        .execute(exe)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;

    use crate::repo::test_util;

    use super::*;

    #[sqlx::test]
    async fn test_crud(pool: PgPool) {
        test_util::migrate(&pool).await;

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

    #[sqlx::test]
    async fn test_find_by_range(pool: PgPool) {
        test_util::migrate(&pool).await;

        // No items
        {
            let items = find_by_range(&pool, Uuid::max(), 100).await.unwrap();
            assert_eq!(items.len(), 0);
        }

        let mut ids = vec![];
        for i in 0..10 {
            let params = ItemParams {
                title: format!("example{}", i),
                url: format!("https://example{}.com/", i),
                thumbnail: format!("https://example{}.com/", i),
            };
            let id = insert(&pool, params).await.unwrap();
            ids.push(id);
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
