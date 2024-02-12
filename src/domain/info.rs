use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Info {
    pub title: String,
    pub thumbnail: String,
}
