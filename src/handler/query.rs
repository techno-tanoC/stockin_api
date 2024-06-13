use std::sync::OnceLock;

use axum::extract::State;
use scraper::{Html, Selector};
use serde::Deserialize;
use tracing::instrument;

use crate::{domain::info::Info, AppState};

use super::{
    request::Json,
    response::{JsonData, Result},
};

#[derive(Debug, Clone, Deserialize)]
pub struct UrlParams {
    url: String,
}

#[instrument(skip(state))]
pub async fn info(state: State<AppState>, Json(params): Json<UrlParams>) -> Result<JsonData<Info>> {
    let document = state.client.get(&params.url).send().await?.text().await?;
    let html = Html::parse_document(&document);

    static TITLE_SELECTOR: OnceLock<Selector> = OnceLock::new();
    let title_selector = TITLE_SELECTOR.get_or_init(|| {
        Selector::parse("html > head > title").expect("Invalid CSS selector for titles")
    });

    let title = html
        .select(title_selector)
        .next()
        .map(|e| e.text().collect::<Vec<_>>().concat())
        .unwrap_or("".to_string())
        .trim()
        .to_string();

    static THUMBNAIL_SELECTOR: OnceLock<Selector> = OnceLock::new();
    let thumbnail_selector = THUMBNAIL_SELECTOR.get_or_init(|| {
        Selector::parse(r#"html > head > meta[property="og:image"]"#)
            .expect("Invalid CSS selector for thumbnails")
    });

    let thumbnail = html
        .select(thumbnail_selector)
        .next()
        .and_then(|e| e.attr("content"))
        .unwrap_or("")
        .to_string()
        .trim()
        .to_string();

    let info = Info { title, thumbnail };
    JsonData::ok(info)
}
