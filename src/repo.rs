pub mod item;
mod item_model;
mod model_ext;

use anyhow::Result;
use chrono::{DateTime, Duration, DurationRound as _, Utc};

struct Timestamp;

impl Timestamp {
    fn now() -> Result<DateTime<Utc>> {
        let now = Utc::now().duration_trunc(Duration::microseconds(1))?;
        Ok(now)
    }
}

#[cfg(test)]
pub mod test_util {
    use sqlx::PgPool;

    pub async fn migrate(pool: &PgPool) {
        let schema = tokio::fs::read_to_string("./schema.sql").await.unwrap();
        sqlx::query(&schema).execute(pool).await.unwrap();
    }
}
