use anyhow::Result;
use chrono::{Duration, DurationRound, NaiveDateTime, TimeZone as _, Utc};
use uuid::{fmt::Hyphenated, Uuid};

use crate::domain::item::{Item, ItemParams};

use super::model_ext::ModelExt;

#[derive(Debug, Clone)]
pub(super) struct Model {
    pub id: String,
    pub title: String,
    pub url: String,
    pub thumbnail: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
            created_at: Utc.from_utc_datetime(&self.created_at),
            updated_at: Utc.from_utc_datetime(&self.updated_at),
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl InsertModel {
    pub fn new(params: ItemParams) -> Result<Self> {
        let id = Uuid::now_v7().hyphenated();
        let now = Utc::now()
            .duration_trunc(Duration::microseconds(1))?
            .naive_utc();
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
    pub updated_at: NaiveDateTime,
}

impl UpdateModel {
    pub fn new(id: Hyphenated, params: ItemParams) -> Result<Self> {
        let now = Utc::now()
            .duration_trunc(Duration::microseconds(1))?
            .naive_utc();
        Ok(Self {
            id,
            title: params.title,
            url: params.url,
            thumbnail: params.thumbnail,
            updated_at: now,
        })
    }
}
