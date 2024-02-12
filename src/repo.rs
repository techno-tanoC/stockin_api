pub mod item;
mod item_model;
mod model_ext;

#[cfg(test)]
pub mod test_util {
    use sqlx::SqlitePool;

    pub async fn connect() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite://:memory:").await.unwrap();

        let schema = tokio::fs::read_to_string("./schema.sql").await.unwrap();
        sqlx::query(&schema).execute(&pool).await.unwrap();

        pool
    }
}
