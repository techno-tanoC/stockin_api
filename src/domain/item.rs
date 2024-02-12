use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::fmt::Hyphenated;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Item {
    pub id: Hyphenated,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ItemParams {
    pub title: String,
    pub url: String,
    pub thumbnail: String,
}
