use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::{fmt::Hyphenated, Uuid};

use crate::domain::item::{Item, ItemParams};

use super::{model_ext::ModelExt, Timestamp};

#[derive(Debug, Clone)]
pub(super) struct Model {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ModelExt for Model {
    type Target = Item;

    fn convert(self) -> Result<Self::Target> {
        let id = Uuid::try_parse(&self.id)?;
        let item = Item {
            id,
            title: self.title,
            url: self.url,
            thumbnail: self.thumbnail,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
        Ok(item)
    }
}

#[derive(Debug, Clone)]
pub(super) struct InsertModel {
    pub id: Hyphenated,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl InsertModel {
    pub fn new(params: ItemParams) -> Result<Self> {
        let id = Uuid::now_v7().hyphenated();
        let now = Timestamp::now()?;
        let model = Self {
            id,
            title: params.title,
            url: params.url,
            thumbnail: params.thumbnail,
            created_at: now,
            updated_at: now,
        };
        Ok(model)
    }
}

#[derive(Debug, Clone)]
pub(super) struct UpdateModel {
    pub id: Hyphenated,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub updated_at: DateTime<Utc>,
}

impl UpdateModel {
    pub fn new(id: Hyphenated, params: ItemParams) -> Result<Self> {
        let now = Timestamp::now()?;
        Ok(Self {
            id,
            title: params.title,
            url: params.url,
            thumbnail: params.thumbnail,
            updated_at: now,
        })
    }
}
