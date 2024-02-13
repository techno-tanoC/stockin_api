use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemId {
    pub item_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemRange {
    from: Option<Uuid>,
    limit: Option<i64>,
}

impl ItemRange {
    pub fn extract(&self) -> (Uuid, i64) {
        let from = self.from.unwrap_or_else(Uuid::max);
        let limit = self.limit.unwrap_or(20).clamp(1, 50);
        (from, limit)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemParams {
    pub title: String,
    pub url: String,
    pub thumbnail: String,
}
