pub mod item;
mod item_model;
mod model_ext;

use anyhow::Result;
use chrono::{Duration, DurationRound as _, NaiveDateTime, Utc};

struct Timestamp;

impl Timestamp {
    fn now() -> Result<NaiveDateTime> {
        let now = Utc::now()
            .duration_trunc(Duration::microseconds(1))?
            .naive_utc();
        Ok(now)
    }
}

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
