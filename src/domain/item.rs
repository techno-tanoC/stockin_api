use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemId {
    pub item_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ItemRange {
    before: Option<Uuid>,
    limit: Option<i64>,
}

impl ItemRange {
    pub fn extract(&self) -> (Uuid, i64) {
        let before = self.before.unwrap_or_else(Uuid::max);
        let limit = self.limit.unwrap_or(20).clamp(1, 50);
        (before, limit)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_range() {
        {
            let range = ItemRange {
                before: None,
                limit: None,
            };
            assert_eq!(range.extract(), (Uuid::max(), 20));
        }

        {
            let range = ItemRange {
                before: Some(Uuid::nil()),
                limit: None,
            };
            assert_eq!(range.extract(), (Uuid::nil(), 20));
        }

        {
            let range = ItemRange {
                before: None,
                limit: Some(0),
            };
            assert_eq!(range.extract(), (Uuid::max(), 1));
        }

        {
            let range = ItemRange {
                before: None,
                limit: Some(10),
            };
            assert_eq!(range.extract(), (Uuid::max(), 10));
        }

        {
            let range = ItemRange {
                before: None,
                limit: Some(100),
            };
            assert_eq!(range.extract(), (Uuid::max(), 50));
        }
    }
}
